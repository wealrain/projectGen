use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug,Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
}

impl Property {
    pub fn new(name: String, value: String) -> Property {
        Property {
            name,
            value,
        }
    }
}


#[derive(Debug,Deserialize,Clone)]
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

#[derive(Debug,Deserialize,Clone)]
pub struct Dependency {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    pub artifact_type: Option<String>,
    pub classifier: Option<String>,
    pub scope: Option<String>,
    pub optional: Option<bool>,
    pub exclusions: Option<Vec<Exclusion>>,
}

impl Dependency {
    pub fn new(group_id: String, artifact_id: String, version: String) -> Dependency {
        Dependency {
            group_id,
            artifact_id,
            version,
            artifact_type: None,
            classifier: None,
            scope: None,
            optional: None,
            exclusions: None,
        }
    }

    pub fn set_artifact_type(mut self, artifact_type: String) -> Dependency {
        self.artifact_type = Some(artifact_type);
        self
    }

    pub fn set_classifier(mut self, classifier: String) -> Dependency {
        self.classifier = Some(classifier);
        self
    }

    pub fn set_scope(mut self, scope: String) -> Dependency {
        self.scope = Some(scope);
        self
    }

    pub fn set_optional(mut self, optional: bool) -> Dependency {
        self.optional = Some(optional);
        self
    }

    pub fn set_exclusions(mut self, exclusions: Vec<Exclusion>) -> Dependency {
        self.exclusions = Some(exclusions);
        self
    }

}


#[derive(Debug,Clone)]
pub struct MavenRepository {
    pub id: String,
    pub url: String,
    pub name: String,
    pub releases_enabled: bool,
    pub snapshots_enabled: bool, 
}

impl MavenRepository {
    pub fn new(id: String, url: String, name: String) -> MavenRepository {
        MavenRepository {
            id,
            url,
            name,
            releases_enabled: true,
            snapshots_enabled: false,
        }
    }

    pub fn maven_center() -> MavenRepository {
        MavenRepository::new("maven-central".to_string(), "https://repo.maven.apache.org/maven2".to_string(), "Maven Central".to_string())
    }

    pub fn set_releases_enabled(mut self, releases_enabled: bool) -> MavenRepository {
        self.releases_enabled = releases_enabled;
        self
    }

    pub fn set_snapshots_enabled(mut self, snapshots_enabled: bool) -> MavenRepository {
        self.snapshots_enabled = snapshots_enabled;
        self
    }

    pub fn only_releases(mut self) -> MavenRepository {
        self.releases_enabled = true;
        self.snapshots_enabled = false;
        self
    }

    pub fn only_snapshots(mut self) -> MavenRepository {
        self.releases_enabled = false;
        self.snapshots_enabled = true;
        self
    }
}

pub struct POMBuilder {
    pub properties: Vec<Property>,
    pub dependencies: Vec<Dependency>,
    pub repositories: Vec<MavenRepository>,
    pub plugin_repositories: Vec<MavenRepository>,
}

impl POMBuilder {
    pub fn new() -> POMBuilder {
        POMBuilder {
            properties: vec![],
            dependencies: vec![],
            repositories: vec![],
            plugin_repositories: vec![],
        }
    }

    pub fn add_property(mut self, property: Property) -> POMBuilder {
        self.properties.push(property);
        self
    }

    pub fn add_dependency(mut self, dependency: Dependency) -> POMBuilder {
        self.dependencies.push(dependency);
        self
    }

    pub fn add_repository(mut self, repository: MavenRepository) -> POMBuilder {
        self.repositories.push(repository);
        self
    }

    pub fn add_plugin_repository(mut self, repository: MavenRepository) -> POMBuilder {
        self.plugin_repositories.push(repository);
        self
    }

    pub fn write(self,path: PathBuf) {

    }

}