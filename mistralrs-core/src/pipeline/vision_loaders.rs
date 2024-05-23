use std::{fmt::Debug, str::FromStr};

use anyhow::Result;
use candle_core::Device;
use candle_nn::VarBuilder;

#[cfg(feature = "pyo3_macros")]
use pyo3::pyclass;

use serde::Deserialize;

use super::VisionModel;
use crate::vision_models::idefics2::{Config as Idefics2Config, Idefics2};
use crate::DeviceMapMetadata;

pub trait VisionModelLoader {
    fn load(
        &self,
        config: &str,
        use_flash_attn: bool,
        vb: VarBuilder,
        mapper: DeviceMapMetadata,
        loading_isq: bool,
        device: Device,
    ) -> Result<Box<dyn VisionModel + Send + Sync>>;
    fn is_gptx(&self) -> bool;
    fn get_config_repr(&self, config: &str, use_flash_attn: bool) -> Result<Box<dyn Debug>>;
}

#[cfg_attr(feature = "pyo3_macros", pyclass)]
#[derive(Clone, Debug, Deserialize)]
/// The architecture to load the vision model as.
pub enum VisionLoaderType {
    #[serde(rename = "idefics2")]
    Idefics2,
}

impl FromStr for VisionLoaderType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "idefics2" => Ok(Self::Idefics2),
            a => Err(format!("Unknown architecture `{a}`")),
        }
    }
}

// ======================== Idefics 2 loader

pub struct Idefics2Loader;

impl VisionModelLoader for Idefics2Loader {
    fn load(
        &self,
        config: &str,
        use_flash_attn: bool,
        vb: VarBuilder,
        mapper: DeviceMapMetadata,
        loading_isq: bool,
        device: Device,
    ) -> Result<Box<dyn VisionModel + Send + Sync>> {
        let mut config: Idefics2Config = serde_json::from_str(config)?;
        config.text_config.use_flash_attn = use_flash_attn;
        Ok(Box::new(Idefics2::new(
            &config,
            vb,
            self.is_gptx(),
            mapper,
            loading_isq,
            device,
        )?))
    }
    fn is_gptx(&self) -> bool {
        true
    }
    fn get_config_repr(&self, config: &str, use_flash_attn: bool) -> Result<Box<dyn Debug>> {
        let mut config: Idefics2Config = serde_json::from_str(config)?;
        config.text_config.use_flash_attn = use_flash_attn;
        Ok(Box::new(config))
    }
}