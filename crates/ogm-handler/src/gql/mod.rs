use serde::de::DeserializeOwned;



pub mod querys;

pub fn json_to_vars<Vars>(input_json_str: &str) -> serde_json::Result<Vars> where Vars: DeserializeOwned {
    serde_json::from_str::<Vars>(input_json_str)
}