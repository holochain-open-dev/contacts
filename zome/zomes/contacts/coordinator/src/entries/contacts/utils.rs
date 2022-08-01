use hdk::prelude::*;

pub(crate) fn error<T>(reason: &str) -> ExternResult<T> {
    Err(wasm_error!(WasmErrorInner::Guest(String::from(reason))))
}


