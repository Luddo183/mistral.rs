var searchIndex = new Map(JSON.parse('[\
["mistralrs",{"doc":"","t":"FFNNNNNNNNNNNNNNNNNNNNNNHNNNNNHOOHONNNNNNNNNNN","n":["Args","RawRequest","augment_args","augment_args_for_update","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","command","command_for_update","deref","deref","deref_mut","deref_mut","deserialize","drop","drop","from","from","from_arg_matches","from_arg_matches_mut","from_ref","get_router","group_id","init","init","into","into","main","port","prompt","root","sampling_params","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","update_from_arg_matches","update_from_arg_matches_mut","vzip","vzip"],"q":[[0,"mistralrs"],[46,"clap_builder::builder::command"],[47,"core::result"],[48,"serde::de"],[49,"clap_builder::parser::matches::arg_matches"],[50,"clap_builder"],[51,"mistralrs_core"],[52,"alloc::sync"],[53,"axum::routing"],[54,"clap_builder::util::id"],[55,"core::option"],[56,"anyhow"],[57,"axum::extract::state"],[58,"axum::json"],[59,"alloc::string"],[60,"core::any"]],"d":["","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","Port to serve on.","","","","","","","","","","","","","",""],"i":[0,0,8,8,8,2,8,2,2,2,8,8,8,2,8,2,2,8,2,8,2,8,8,2,0,8,8,2,8,2,0,8,2,0,2,2,8,2,8,2,8,2,8,8,8,2],"f":[0,0,[1,1],[1,1],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[2,2],[[-1,-2],3,[],[]],[[],1],[[],1],[4,-1,[]],[4,-1,[]],[4,-1,[]],[4,-1,[]],[-1,[[5,[2]]],6],[4,3],[4,3],[-1,-1,[]],[-1,-1,[]],[7,[[5,[8,9]]]],[7,[[5,[8,9]]]],[-1,-1,[]],[[[11,[10]]],12],[[],[[14,[13]]]],[[],4],[[],4],[-1,-2,[],[]],[-1,-2,[],[]],[[],[[15,[3]]]],0,0,[[[16,[[11,[10]]]],[17,[2]]],18],0,[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,19,[]],[-1,19,[]],[[8,7],[[5,[3,9]]]],[[8,7],[[5,[3,9]]]],[-1,-2,[],[]],[-1,-2,[],[]]],"c":[],"p":[[5,"Command",46],[5,"RawRequest",0],[1,"tuple"],[1,"usize"],[6,"Result",47],[10,"Deserializer",48],[5,"ArgMatches",49],[5,"Args",0],[8,"Error",50],[5,"MistralRs",51],[5,"Arc",52],[5,"Router",53],[5,"Id",54],[6,"Option",55],[8,"Result",56],[5,"State",57],[5,"Json",58],[5,"String",59],[5,"TypeId",60]],"b":[]}],\
["mistralrs_core",{"doc":"","t":"PPPPPKFFFPFGFGGMNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNQQNMNNNNNNNNNNNNNNNNNQNQQNNNNNNNNNNNNNNNNNNONNOOOOOOONNOOONNNNNNNNNNNNNNNNNNNNNNNNONNNNNNNN","n":["CacheToken","Done","EnvVar","Error","Fixed","Loader","MistralLoader","MistralRs","MistralSpecificConfig","Path","Request","Response","SamplingParams","SchedulerMethod","TokenSource","_setup_model","_setup_model","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","deref","deref","deref","deref","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut_refcell","deref_refcell","deserialize","download_model","download_model","drop","drop","drop","drop","drop","drop","drop","drop","from","from","from","from","from","from","from","from","get_mut_arcmutex","get_sender","handle_seq_error","handle_seq_error_stateaware","init","init","init","init","init","init","init","init","into","into","into","into","into","into","into","into","load_model","load_model","max_len","new","new","prompt","repeat_last_n","repeat_penalty","response","sampling_params","stop_toks","temperature","to_owned","to_owned","top_k","top_n_logprobs","top_p","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","use_flash_attn","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip"],"q":[[0,"mistralrs_core"],[144,"candle_core::dtype"],[145,"core::option"],[146,"candle_core::device"],[147,"std::sync::mutex"],[148,"alloc::boxed"],[149,"anyhow"],[150,"core::result"],[151,"serde::de"],[152,"alloc::string"],[153,"std::sync::mpsc"],[154,"alloc::sync"],[155,"core::any"]],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","If <code>revision</code> is None, then it defaults to <code>main</code>. If <code>dtype</code> is …","If <code>revision</code> is None, then it defaults to <code>main</code>. If <code>dtype</code> is …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[16,23,16,23,20,0,0,0,0,16,0,0,0,0,0,1,8,8,16,18,23,20,17,9,10,8,16,18,23,20,17,9,10,9,10,9,10,8,16,18,23,20,17,9,10,8,16,18,23,20,17,9,10,0,0,10,1,8,8,16,18,23,20,17,9,10,8,16,18,23,20,17,9,10,0,17,0,0,8,16,18,23,20,17,9,10,8,16,18,23,20,17,9,10,1,1,10,8,17,18,9,10,18,18,10,10,9,10,10,10,10,8,16,18,23,20,17,9,10,8,16,18,23,20,17,9,10,8,16,18,23,20,17,9,10,9,8,16,18,23,20,17,9,10],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[9,9],[10,10],[[-1,-2],11,[],[]],[[-1,-2],11,[],[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],[12,-1,[]],0,0,[-1,[[13,[10]]],14],[[1,[3,[15]],16],[[7,[[6,[0]]]]]],[[8,[3,[15]],16],[[7,[[6,[0]]]]]],[12,11],[12,11],[12,11],[12,11],[12,11],[12,11],[12,11],[12,11],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],0,[17,[[19,[18]]]],0,0,[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[[1,[3,[15]],16,[3,[2]],4],[[7,[[6,[[5,[0]]]]]]]],[[1,[3,[15]],16,[3,[2]],4],[[7,[[6,[[5,[0]]]]]]]],0,[[15,9,[3,[2]]],8],[[[6,[[5,[0]]]],20],[[21,[17]]]],0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],0,0,0,[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,[[13,[-2]]],[],[]],[-1,22,[]],[-1,22,[]],[-1,22,[]],[-1,22,[]],[-1,22,[]],[-1,22,[]],[-1,22,[]],[-1,22,[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]]],"c":[],"p":[[10,"Loader",0],[6,"DType",144],[6,"Option",145],[6,"Device",146],[5,"Mutex",147],[5,"Box",148],[8,"Result",149],[5,"MistralLoader",0],[5,"MistralSpecificConfig",0],[5,"SamplingParams",0],[1,"tuple"],[1,"usize"],[6,"Result",150],[10,"Deserializer",151],[5,"String",152],[6,"TokenSource",0],[5,"MistralRs",0],[5,"Request",0],[5,"Sender",153],[6,"SchedulerMethod",0],[5,"Arc",154],[5,"TypeId",155],[6,"Response",0]],"b":[]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
