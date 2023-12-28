use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub enum DataType {
    AutoId,
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    DateTime,
    Object,
    List,
    Ref
}
