use serde::{Deserialize, Serialize};
use crate::Result;
use super::common::DataType;
use std::{fs::File, io::BufReader, collections::HashMap};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct DataDefinition {
    pub datasources: Vec<DataSource>,
    #[serde(skip)]
    pub entity_ref: HashMap<String, Entity>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct DataSource {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub entities: Vec<Entity>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Entity {
    pub name: String,
    pub table: String,
    pub fields: Vec<Field>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Field {
    pub name: String,
    pub column: String,
    #[serde(rename = "type")]
    pub field_type: DataType,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    pub list: Option<TypeList>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TypeList {
    #[serde(rename = "type")]
    pub list_type: DataType,
    #[serde(rename = "ref")]
    pub ref_type: Option<String>,
}

fn default_port() -> u16 {
    3306
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

impl DataDefinition {
    pub fn parse(yaml_path: &str) -> Result<Self> {
        let file = File::open(yaml_path)?;
        let reader = BufReader::new(file);
        let mut definition: DataDefinition = serde_yaml::from_reader(reader)?;
        definition.check()?;
        Ok(definition)
    }

    /// 整理数据将data抽取到Map中，方便其他对象引用
    /// 同时检测是否有数据不正确
    fn check(&mut self) -> Result<()> {
        // todo check
        self.datasources.iter().for_each(|ds|{
           ds.entities.iter().for_each(|x|{
               self.entity_ref.insert(x.name.clone(), x.clone());
           });
        });

        Ok(())
    }

}