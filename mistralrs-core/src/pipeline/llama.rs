use super::{
    get_completion_input, get_model_paths, get_prompt_input, get_xlora_paths, ChatTemplate, Loader,
    ModelKind, ModelPaths, Pipeline, TokenSource, XLoraPaths,
};
use crate::models::llama::MAX_SEQ_LEN;
use crate::models::{quantized_llama, Cache};
use crate::xlora_models::{XLoraConfig, XLoraLlama, XLoraModelWeights};
use crate::{deref_mut_refcell, deref_refcell};
use crate::{
    models::llama::{Llama as NormalModel, LlamaConfig},
    models::quantized_llama::ModelWeights as QModelWeights,
    sequence::Sequence,
    utils::{tokens::get_token, varbuilder_utils::from_mmaped_safetensors},
};
use anyhow::Result;
use candle_core::quantized::{ggml_file, gguf_file};
use candle_core::{DType, Device, Tensor};
use candle_sampling::logits_processor::Logprobs;
use hf_hub::{api::sync::ApiBuilder, Repo, RepoType};
use mistralrs_lora::{LoraConfig, Ordering};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::{rc::Rc, sync::Mutex};
use thiserror::Error;
use tokenizers::Tokenizer;

enum Model {
    Normal(NormalModel),
    Quantized(QModelWeights),
    XLoraNormal(XLoraLlama),
    XLoraQuantized(XLoraModelWeights),
}

pub struct LlamaModelPaths<P> {
    tokenizer_filename: P,
    config_filename: P,
    template_filename: P,
    filenames: Vec<P>,
    xlora_adapter_filenames: Option<Vec<(String, P)>>,
    xlora_adapter_configs: Option<Vec<(String, LoraConfig)>>,
    classifier_path: Option<P>,
    classifier_config: Option<XLoraConfig>,
    xlora_ordering: Option<Ordering>,
}

impl ModelPaths for LlamaModelPaths<PathBuf> {
    fn get_config_filename(&self) -> &PathBuf {
        &self.config_filename
    }
    fn get_tokenizer_filename(&self) -> &PathBuf {
        &self.tokenizer_filename
    }
    fn get_weight_filenames(&self) -> &[PathBuf] {
        &self.filenames
    }
    fn get_adapter_filenames(&self) -> &Option<Vec<(String, PathBuf)>> {
        &self.xlora_adapter_filenames
    }
    fn get_adapter_configs(&self) -> &Option<Vec<(String, LoraConfig)>> {
        &self.xlora_adapter_configs
    }
    fn get_classifier_config(&self) -> &Option<XLoraConfig> {
        &self.classifier_config
    }
    fn get_classifier_path(&self) -> &Option<PathBuf> {
        &self.classifier_path
    }
    fn get_ordering(&self) -> &Option<Ordering> {
        &self.xlora_ordering
    }
    fn get_template_filename(&self) -> &PathBuf {
        &self.template_filename
    }
}

pub struct LlamaPipeline {
    model: Model,
    tokenizer: Tokenizer,
    config: LlamaSpecificConfig,
    no_kv_cache: bool,
    chat_template: ChatTemplate,
}

pub struct LlamaLoader {
    model_id: String,
    config: LlamaSpecificConfig,
    quantized_model_id: Option<String>,
    quantized_filename: Option<String>,
    xlora_model_id: Option<String>,
    kind: ModelKind,
    xlora_order: Option<Ordering>,
    no_kv_cache: bool,
    chat_template: Option<String>,
}

#[derive(Clone, Copy)]
pub struct LlamaSpecificConfig {
    pub repeat_last_n: usize,
    pub use_flash_attn: bool,
    pub gqa: usize,
}

#[derive(Error, Debug)]
enum TokenizerError {
    #[error("`{0}`")]
    Error(String),
}

impl LlamaLoader {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        model_id: String,
        config: LlamaSpecificConfig,
        quantized_model_id: Option<String>,
        quantized_filename: Option<String>,
        xlora_model_id: Option<String>,
        kind: ModelKind,
        xlora_order: Option<Ordering>,
        no_kv_cache: bool,
        chat_template: Option<String>,
    ) -> Self {
        Self {
            model_id,
            config,
            quantized_model_id,
            quantized_filename,
            xlora_model_id,
            kind,
            xlora_order,
            no_kv_cache,
            chat_template,
        }
    }
}

impl Loader for LlamaLoader {
    fn download_model(
        &self,
        revision: Option<String>,
        token_source: TokenSource,
    ) -> Result<Box<dyn ModelPaths>> {
        let api = ApiBuilder::new()
            .with_progress(true)
            .with_token(Some(get_token(&token_source)?))
            .build()?;
        let revision = revision.unwrap_or("main".to_string());
        let api = api.repo(Repo::with_revision(
            self.model_id.clone(),
            RepoType::Model,
            revision.clone(),
        ));

        let tokenizer_filename = api.get("tokenizer.json")?;

        let config_filename = api.get("config.json")?;

        let filenames = get_model_paths(
            revision.clone(),
            &token_source,
            &self.quantized_model_id,
            &self.quantized_filename,
            &api,
        )?;

        let XLoraPaths {
            adapter_configs,
            adapter_safetensors,
            classifier_path,
            xlora_order,
            xlora_config,
        } = get_xlora_paths(
            &self.xlora_model_id,
            &token_source,
            revision.clone(),
            &self.xlora_order,
        )?;

        let template_filename = api.get("tokenizer_config.json")?;

        Ok(Box::new(LlamaModelPaths {
            tokenizer_filename,
            config_filename,
            filenames,
            xlora_adapter_configs: adapter_configs,
            xlora_adapter_filenames: adapter_safetensors,
            classifier_path,
            classifier_config: xlora_config,
            xlora_ordering: xlora_order,
            template_filename,
        }))
    }

    fn _setup_model(
        &self,
        paths: &dyn ModelPaths,
        dtype: Option<DType>,
        device: &Device,
    ) -> Result<Box<Mutex<dyn Pipeline + Send + Sync>>> {
        let basic_config: LlamaConfig =
            serde_json::from_slice(&std::fs::read(paths.get_config_filename())?)?;
        let default_dtype = if device.is_cuda() {
            DType::BF16
        } else {
            DType::F32
        };

        println!("Loading model on {device:?}...");
        let model = match self.kind {
            ModelKind::QuantizedGGUF => {
                let mut file = std::fs::File::open(paths.get_weight_filenames().first().unwrap())?;
                let model = gguf_file::Content::read(&mut file)
                    .map_err(|e| e.with_path(paths.get_weight_filenames().first().unwrap()))?;
                let model = QModelWeights::from_gguf(model, &mut file, device)?;
                Model::Quantized(model)
            }
            ModelKind::QuantizedGGML => {
                let mut file = std::fs::File::open(paths.get_weight_filenames().first().unwrap())?;
                let model = ggml_file::Content::read(&mut file, device)
                    .map_err(|e| e.with_path(paths.get_weight_filenames().first().unwrap()))?;
                let model = QModelWeights::from_ggml(model, self.config.gqa)?;
                Model::Quantized(model)
            }
            ModelKind::Normal => {
                let vb = from_mmaped_safetensors(
                    paths.get_weight_filenames().to_vec(),
                    Vec::new(),
                    dtype.unwrap_or(default_dtype),
                    device,
                    false,
                )?;

                let model = NormalModel::load(
                    vb,
                    &basic_config.into_config(self.config.use_flash_attn),
                    dtype.unwrap_or(default_dtype),
                    device,
                    self.no_kv_cache,
                )?;
                Model::Normal(model)
            }
            ModelKind::XLoraNormal => {
                let mut safetensors_paths = paths.get_weight_filenames().iter().collect::<Vec<_>>();
                safetensors_paths.push(paths.get_classifier_path().as_ref().unwrap());
                let vb = from_mmaped_safetensors(
                    safetensors_paths
                        .iter()
                        .map(|x| (*x).to_owned())
                        .collect::<Vec<_>>(),
                    paths
                        .get_adapter_filenames()
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(_, x)| (*x).to_owned())
                        .collect::<Vec<_>>(),
                    dtype.unwrap_or(default_dtype),
                    device,
                    false,
                )?;

                let model = XLoraLlama::load(
                    vb,
                    &basic_config.into_config(self.config.use_flash_attn),
                    dtype.unwrap_or(default_dtype),
                    device,
                    paths.get_adapter_configs().as_ref().unwrap(),
                    paths.get_classifier_config().as_ref().unwrap().clone(),
                    paths.get_ordering().as_ref().unwrap().clone(),
                    self.no_kv_cache,
                )?;
                Model::XLoraNormal(model)
            }
            ModelKind::XLoraGGUF => {
                let vb = from_mmaped_safetensors(
                    vec![paths.get_classifier_path().as_ref().unwrap().to_path_buf()],
                    paths
                        .get_adapter_filenames()
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(_, x)| (*x).to_owned())
                        .collect::<Vec<_>>(),
                    dtype.unwrap_or(default_dtype),
                    device,
                    false,
                )?;

                let mut file = std::fs::File::open(paths.get_weight_filenames().first().unwrap())?;
                let model = ggml_file::Content::read(&mut file, device)
                    .map_err(|e| e.with_path(paths.get_weight_filenames().first().unwrap()))?;
                let model = XLoraModelWeights::from_ggml(
                    model,
                    self.config.gqa,
                    paths.get_adapter_configs().as_ref().unwrap(),
                    &vb,
                    paths.get_ordering().as_ref().unwrap(),
                    paths.get_classifier_config().as_ref().unwrap().clone(),
                )?;
                Model::XLoraQuantized(model)
            }
            ModelKind::XLoraGGML => {
                let vb = from_mmaped_safetensors(
                    vec![paths.get_classifier_path().as_ref().unwrap().to_path_buf()],
                    paths
                        .get_adapter_filenames()
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(_, x)| (*x).to_owned())
                        .collect::<Vec<_>>(),
                    dtype.unwrap_or(default_dtype),
                    device,
                    false,
                )?;

                let mut file = std::fs::File::open(paths.get_weight_filenames().first().unwrap())?;
                let model = gguf_file::Content::read(&mut file)
                    .map_err(|e| e.with_path(paths.get_weight_filenames().first().unwrap()))?;
                let model = XLoraModelWeights::from_gguf(
                    model,
                    &mut file,
                    device,
                    paths.get_adapter_configs().as_ref().unwrap(),
                    &vb,
                    paths.get_ordering().as_ref().unwrap(),
                    paths.get_classifier_config().as_ref().unwrap().clone(),
                )?;
                Model::XLoraQuantized(model)
            }
        };
        println!("Model loaded.");

        let tokenizer = Tokenizer::from_file(paths.get_tokenizer_filename())
            .map_err(|e| TokenizerError::Error(e.to_string()))?;

        let chat_template: ChatTemplate = match serde_json::from_str(&fs::read_to_string(
            paths.get_template_filename(),
        )?) {
            Ok(template) => template,
            Err(_) => {
                println!("Deserializing chat template failed, attempting to use specified JINJA template");
                let mut deser: HashMap<String, Value> =
                    serde_json::from_str(&fs::read_to_string(paths.get_template_filename())?)
                        .unwrap();
                deser.insert(
                    "chat_template".to_string(),
                    Value::String(
                        self.chat_template
                            .clone()
                            .expect("Please specify a manual chat template."),
                    ),
                );
                let ser = serde_json::to_string_pretty(&deser).unwrap();
                serde_json::from_str(&ser).unwrap()
            }
        };

        Ok(Box::new(Mutex::new(LlamaPipeline {
            model,
            tokenizer,
            config: self.config,
            no_kv_cache: self.no_kv_cache,
            chat_template,
        })))
    }
}

impl Pipeline for LlamaPipeline {
    fn forward(&mut self, input_toks: Box<[Rc<RefCell<Sequence>>]>, is_prompt: bool) -> Tensor {
        let (input_ids, input_ids_full, seqlen_offsets, seqlen_offsets_full) =
            if self.is_xlora() && !is_prompt {
                let (input_ids_full, seqlen_offsets_full) =
                    get_prompt_input(&input_toks, self.device());
                let (input_ids, seqlen_offsets) =
                    get_completion_input(&input_toks, self.device(), self.no_kv_cache);
                (
                    input_ids,
                    Some(input_ids_full),
                    seqlen_offsets,
                    Some(seqlen_offsets_full),
                )
            } else if self.is_xlora() && is_prompt {
                let (input_ids_full, seqlen_offsets) = get_prompt_input(&input_toks, self.device());
                (
                    input_ids_full.clone(),
                    Some(input_ids_full),
                    seqlen_offsets.clone(),
                    Some(seqlen_offsets),
                )
            } else if is_prompt {
                let (input_ids, seqlen_offsets) = get_prompt_input(&input_toks, self.device());
                (input_ids, None, seqlen_offsets, None)
            } else {
                let (input_ids, seqlen_offsets) =
                    get_completion_input(&input_toks, self.device(), self.no_kv_cache);
                (input_ids, None, seqlen_offsets, None)
            };
        let result = match self.model {
            Model::Normal(ref mut model) => model.forward(&input_ids, &seqlen_offsets),
            Model::Quantized(ref mut model) => model.forward(&input_ids, &seqlen_offsets),
            Model::XLoraNormal(ref mut model) => model.forward(
                &input_ids,
                input_ids_full.as_ref().unwrap(),
                &seqlen_offsets,
                seqlen_offsets_full.as_ref().unwrap(),
                self.no_kv_cache,
            ),
            Model::XLoraQuantized(ref mut model) => model.forward(
                &input_ids,
                input_ids_full.as_ref().unwrap(),
                &seqlen_offsets,
                seqlen_offsets_full.as_ref().unwrap(),
                self.no_kv_cache,
            ),
        };
        match result {
            Ok(v) => v,
            Err(e) => {
                panic!("Model failed with error `{e}`. Please raise an issue.");
            }
        }
    }
    fn tokenize_prompt(&self, prompt: &str) -> Result<Vec<u32>> {
        let encoding = self
            .tokenizer
            .encode(prompt, false)
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        Ok(encoding.get_ids().to_vec())
    }
    fn device(&self) -> &Device {
        match self.model {
            Model::Normal(ref model) => &model.device,
            Model::Quantized(ref model) => &model.device,
            Model::XLoraNormal(ref model) => &model.device,
            Model::XLoraQuantized(ref model) => &model.device,
        }
    }
    fn num_hidden_layers(&self) -> usize {
        self.cache().lock().len()
    }
    fn cache(&self) -> &Cache {
        match self.model {
            Model::Normal(ref model) => &model.kv_cache,
            Model::Quantized(ref model) => &model.cache,
            Model::XLoraNormal(ref model) => &model.kv_cache,
            Model::XLoraQuantized(ref model) => &model.cache,
        }
    }
    fn sample(&mut self, logits: Tensor, seq: Rc<RefCell<Sequence>>) -> Result<Logprobs> {
        let logits = logits
            .squeeze(0)
            .unwrap()
            .squeeze(0)
            .unwrap()
            .to_dtype(DType::F32)
            .unwrap();
        let start_at = deref_refcell!(seq)
            .get_toks()
            .len()
            .saturating_sub(self.config.repeat_last_n);
        let ctxt = deref_refcell!(seq).get_toks()[start_at..].to_vec();

        Ok(deref_mut_refcell!(seq)
            .logits_processor()
            .sample(&logits, Some(&ctxt))?)
    }
    fn tokenizer(&self) -> Tokenizer {
        self.tokenizer.clone()
    }
    fn eos_tok(&self) -> u32 {
        self.tokenizer
            .get_vocab(true)
            .get("</s>")
            .copied()
            .expect("Unable to extract `</s>` EOS token.")
    }
    fn name(&self) -> &'static str {
        "llama"
    }
    fn get_max_seq_len(&self) -> usize {
        match &self.model {
            Model::Normal(_) | Model::XLoraNormal(_) => MAX_SEQ_LEN,
            Model::Quantized(_) | Model::XLoraQuantized(_) => quantized_llama::MAX_SEQ_LEN as usize,
        }
    }
    fn is_xlora(&self) -> bool {
        match &self.model {
            Model::Normal(_) | Model::Quantized(_) => false,
            Model::XLoraNormal(_) | Model::XLoraQuantized(_) => true,
        }
    }
    fn has_no_kv_cache(&self) -> bool {
        self.no_kv_cache
    }
    fn get_chat_template(&self) -> &ChatTemplate {
        &self.chat_template
    }
}