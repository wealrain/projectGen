use crate::java::Dependency;
use serde::Deserialize;

#[derive(Debug, Deserialize,Clone)]
pub struct Config {
    pub java_config: JavaConfig,
}

#[derive(Debug,Deserialize,Clone)]
pub struct JavaConfig {
    pub jdk_version: String,
    pub pom_parent: Dependency,
    pub pom_dependencies: Vec<Dependency>,
    pub pom_plugins: Vec<Dependency>,
}