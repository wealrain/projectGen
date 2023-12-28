const SPRING_BOOT_VERSION: &str = "2.6.6";

#[derive(Debug,Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
}

impl Property {
    pub fn new(name: &str, value: &str) -> Property {
        Property {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
}


#[derive(Debug,Clone)]
pub struct Exclusion {
    pub group_id: String,
    pub artifact_id: String,
}

impl Exclusion {
    pub fn new(group_id: String, artifact_id: String) -> Exclusion {
        Exclusion {
            group_id,
            artifact_id,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Dependency {
    pub group_id: String,
    pub artifact_id: String,
    pub version: Option<String>,
    pub artifact_type: Option<String>,
    pub classifier: Option<String>,
    pub scope: Option<String>,
    pub optional: Option<bool>,
    pub exclusions: Option<Vec<Exclusion>>,
}

impl Dependency {
    pub fn new(group_id: &str, artifact_id: &str, version:Option<&str>) -> Dependency {
        Dependency {
            group_id:group_id.to_owned(),
            artifact_id:artifact_id.to_owned(),
            version:version.map(|s|s.to_owned()),
            artifact_type: None,
            classifier: None,
            scope: None,
            optional: None,
            exclusions: None,
        }
    }

    pub fn set_artifact_type(&mut self, artifact_type: String) -> &Self {
        self.artifact_type = Some(artifact_type);
        self
    }

    pub fn set_classifier(&mut self, classifier: String) -> &Self {
        self.classifier = Some(classifier);
        self
    }

    pub fn set_scope(&mut self, scope: String) -> &Self {
        self.scope = Some(scope);
        self
    }

    pub fn set_optional(&mut self, optional: bool) -> &Self {
        self.optional = Some(optional);
        self
    }

    pub fn set_exclusions(&mut self, exclusions: Vec<Exclusion>) -> &Self {
        self.exclusions = Some(exclusions);
        self
    }

}

pub struct Plugin {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    //todo 
}

pub struct Build {
    pub plugins: Vec<Plugin>,
}

impl Default for Build {
    fn default() -> Self {
        Build {
            plugins: vec![Plugin{
                group_id: "org.springframework.boot".to_string(),
                artifact_id: "spring-boot-maven-plugin".to_string(),
                version: SPRING_BOOT_VERSION.to_string(),
            }]
        }
    }
}
 
pub struct POM {
    pub model_version: String,
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    pub packaging: Option<String>,
    pub parent: Option<Dependency>,
    pub properties: Vec<Property>,
    pub dependencies: Vec<Dependency>,
    pub build: Option<Build>,
    
}

impl POM {
    pub fn new(group_id: &str,artifact_id: &str) -> Self {
        POM {
            model_version: "4.0.0".to_string(),
            group_id: group_id.to_string(),
            artifact_id: artifact_id.to_string(),
            version: "0.0.1".to_string(),
            parent: Some(Self::spring_boot_parent()),
            packaging: None,
            properties: vec![],
            dependencies: vec![],
            build: Some(Build::default()),
        }
    }

    fn spring_boot_parent() -> Dependency {
        Dependency::new(
        "org.springframework.boot", 
        "spring-boot-starter-parent", 
                Some(SPRING_BOOT_VERSION))
    }

    pub fn set_packaging(&mut self, packaging: String) -> &Self {
        self.packaging = Some(packaging);
        self
    }
    
     
    pub fn add_property(&mut self, property: Property) -> &Self {
        self.properties.push(property);
        self
    }

    pub fn add_dependency(&mut self, dependency: Dependency) -> &Self {
        self.dependencies.push(dependency);
        self
    }
 

}