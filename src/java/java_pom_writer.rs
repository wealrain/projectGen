use std::{io::Write, fs::{OpenOptions, File}, path::PathBuf};

use quick_xml::{writer::Writer, events::{BytesStart, Event, BytesEnd, BytesText}, Error};

use crate::Result;

use super::{POM, Dependency, Property, Build, Plugin};

const PROJECT_XMLNS : &str = "http://maven.apache.org/POM/4.0.0";
const PROJECT_XSI : &str = "http://www.w3.org/2001/XMLSchema-instance";
const PROJECT_XSD : &str = "http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd";

pub struct POMWriter {
    writer: Writer<Vec<u8>>
}

impl Default for POMWriter {
    fn default() -> Self {
        let writer = Writer::new_with_indent(Vec::new(),b' ',4);
        POMWriter{
            writer,
        }
    }
}

impl POMWriter {
    
    fn write(&mut self,pom: &POM) -> Result<()> {
       self.write_root_start()?;
       self.write_parent(pom.parent.as_ref())?;
       self.writer.create_element("modelVersion").write_text_content(BytesText::new(pom.model_version.as_str()))?;
       self.writer.create_element("groupId").write_text_content(BytesText::new(pom.group_id.as_str()))?;
       self.writer.create_element("artifactId").write_text_content(BytesText::new(pom.artifact_id.as_str()))?;
       self.writer.create_element("version").write_text_content(BytesText::new(pom.version.as_str()))?;
    
       self.write_properties(&pom.properties)?;
       self.write_dependenies(&pom.dependencies)?;
       self.write_build(pom.build.as_ref())?;
       self.write_root_end()?;
       Ok(())
    }

    fn write_parent(&mut self,parent: Option<&Dependency>) -> Result<()> {
        match parent {
            Some(p) => {
                self.writer.write_event(Event::Start(BytesStart::new("parent")))?;
                self.write_dependency(p)?;
                self.writer.write_event(Event::End(BytesEnd::new("parent")))?;
                Ok(())
            },
            None => Ok(())
        }
    }

    fn write_properties(&mut self,properties: &Vec<Property>) -> Result<()> {
        if properties.len() == 0 {
            return Ok(())
        }
        self.writer.create_element("properties")
            .write_inner_content::<_,Error>(|w| {
                for p in properties {
                    w.create_element(p.name.as_str())
                       .write_text_content(BytesText::new(p.value.as_str()))?;
                }
                Ok(())
            })?;
        Ok(())
    }

    fn write_dependenies(&mut self,dependencies: &Vec<Dependency>) -> Result<()> {
        if dependencies.len() == 0 {
            return Ok(())
        }
        self.writer.write_event(Event::Start(BytesStart::new("dependencies")))?;
        for d in dependencies {
            self.writer.write_event(Event::Start(BytesStart::new("dependency")))?;
            self.write_dependency(&d)?;
            self.writer.write_event(Event::End(BytesEnd::new("dependency")))?;
        }
        self.writer.write_event(Event::End(BytesEnd::new("dependencies")))?;
        Ok(())
    }

    fn write_build(&mut self,build: Option<&Build>) -> Result<()> {
        match build {
            Some(b) => {
                self.writer.write_event(Event::Start(BytesStart::new("build")))?;
                self.write_build_plugins(&b.plugins)?;
                self.writer.write_event(Event::End(BytesEnd::new("build")))?;
                Ok(())
            },
            None => Ok(())
        }  
    }

    fn write_build_plugins(&mut self,plugins: &Vec<Plugin>) -> Result<()> {
        if plugins.len() == 0 {
            return Ok(())
        }
        self.writer.write_event(Event::Start(BytesStart::new("plugins")))?;
        for p in plugins {
            self.write_plugin(p)?;
        }
        self.writer.write_event(Event::End(BytesEnd::new("plugins")))?;
        Ok(())
    }

    fn write_plugin(&mut self,plugin: &Plugin) ->Result<()>{
        self.writer.write_event(Event::Start(BytesStart::new("plugin")))?;
        self.writer.create_element("groupId").write_text_content(BytesText::new(plugin.group_id.as_str()))?;
        self.writer.create_element("artifactId").write_text_content(BytesText::new(plugin.artifact_id.as_str()))?;
        self.writer.create_element("version").write_text_content(BytesText::new(plugin.version.as_str()))?;
        self.writer.write_event(Event::End(BytesEnd::new("plugin")))?;
        Ok(()) 
    }


    fn write_dependency(&mut self,dependency: &Dependency) -> Result<()> {
        self.writer.create_element("groupId").write_text_content(BytesText::new(dependency.group_id.as_str()))?;
        self.writer.create_element("artifactId").write_text_content(BytesText::new(dependency.artifact_id.as_str()))?;
        if let Some(v) = dependency.version.as_ref() {
            self.writer.create_element("version").write_text_content(BytesText::new(v.as_str()))?;
        }

        Ok(())
    }

    fn write_root_start(&mut self) -> Result<()> {
        let mut elem = BytesStart::new("project");
        elem.push_attribute(("xmlns",PROJECT_XMLNS));
        elem.push_attribute(("xmlns:xsi",PROJECT_XSI));
        elem.push_attribute(("xsi:schemaLocation",PROJECT_XSD));
        self.writer.write_event(Event::Start(elem))?;
        Ok(())
    }

    fn write_root_end(&mut self) -> Result<()> {
        self.writer.write_event(Event::End(BytesEnd::new("project")))?;
        Ok(())
    }

    pub fn write_to(&mut self,pom: &POM,directory:PathBuf) -> Result<()>{
        let file_name = "pom.xml";
        let to = directory.join(file_name);
        let mut file:File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(to)
        .map(|e| e.into())?;
        self.write(pom)?;
        write!(&mut file,"{}",std::str::from_utf8(self.writer.clone().into_inner().as_slice())?)?;
        Ok(())
    }


}