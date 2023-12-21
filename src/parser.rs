use std::{fs::File, io::BufReader, collections::HashMap};

use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Serialize, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub enum DataKind {
    Db,
    Dto
}

#[derive(Debug, Serialize, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub enum DataType {
    Id,
    String,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Bool,
    DateTime,
    Object,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser {
    pub project: String,
    pub dir: Option<String>,
    #[serde(rename = "basePackage")]
    pub base_package: String,
    pub group: Option<String>,
    pub git: String,
    #[serde(rename = "datasource")]
    pub data_source: Option<DataSource>,
    data: Option<Vec<Data>>,
    pub api: Vec<Api>,
    #[serde(skip)]
    pub data_ref: HashMap<String, Data>,
}

/// 定义项目数据源
#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    pub ip: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String
}

/// 定义项目数据包括实体数据和dto数据
/// 此处定义的数据一般会被其他项目引用
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Data {
    pub name: String,
    pub kind: DataKind,
    pub table: Option<String>,
    pub props: Vec<DataProp>
}

/// 定义数据属性 
/// type、ref、list 三个同时只能出现一个
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct DataProp {
    pub name: String,
    #[serde(rename = "type")]
    pub prop_type: Option<DataType>,
    pub db: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    pub list: Option<TypeList>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TypeList {
    #[serde(rename = "type")]
    pub list_type: Option<DataType>,
    #[serde(rename = "ref")]
    pub ref_type: Option<String>,
}

/// 定义接口
#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    pub name: String,
    #[serde(rename = "baseUrl")]
    pub base_url: Option<String>,
    pub requests: Vec<ApiRequest>,
}

/// 定义接口请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequest {
    pub name: String,
    pub method: String,
    pub path: Option<String>,
    pub params: Option<Vec<ApiRequestParamter>>,

}

/// 定义接口请求参数
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequestParamter {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub param_type: Option<DataType>,
    #[serde(rename = "pathVariable")]
    pub path_variable: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    pub list: Option<TypeList>
}

fn default_port() -> u16 {
    3306
}

impl Parser {
    pub fn parse(yaml_path: &str) -> Result<Self> {
        let file = File::open(yaml_path)?;
        let reader = BufReader::new(file);
        let mut config: Parser = serde_yaml::from_reader(reader)?;
        config.check()?;
        Ok(config)
    }

    /// 整理数据将data抽取到Map中，方便其他对象引用
    /// 同时检测是否有数据不正确
    fn check(&mut self) -> Result<()> {
        // todo check
        if let Some(data) = self.data.as_ref() {
            data.iter().for_each(|x|{
                self.data_ref.insert(x.name.clone(), x.clone());
            });
        }
        Ok(())
    }

}
