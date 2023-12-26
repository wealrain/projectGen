use std::{
    path::PathBuf, 
    fs::{File, OpenOptions, self}, 
    io::Write,  fmt, collections::HashSet, 
};

use serde::de::value;

use crate::{Result, parse};

pub struct Modifier {
    pub modifiers: u16,
}

impl Modifier {
    pub fn new(modifiers: u16) -> Modifier {
        Modifier { modifiers }
    }

    fn is_public(&self) -> bool {
        self.modifiers & 0b10 != 0
    }

    fn is_private(&self) -> bool {
        self.modifiers & 0b100!= 0
    }

    fn is_protected(&self) -> bool {
        self.modifiers & 0b1000!= 0
    }

    fn is_static(&self) -> bool {
        self.modifiers & 0b10000!= 0
    }

    fn is_final(&self) -> bool {
        self.modifiers & 0b100000!= 0
    }

    fn is_abstract(&self) -> bool {
        self.modifiers & 0b1000000!= 0
    }

    fn is_native(&self) -> bool {
        self.modifiers & 0b10000000!= 0
    }

    fn is_strictfp(&self) -> bool {
        self.modifiers & 0b100000000!= 0   
    }

    fn is_synchronized(&self) -> bool {
        self.modifiers & 0b1000000000!= 0
    }

    fn is_transient(&self) -> bool {
        self.modifiers & 0b10000000000!= 0
    }

    fn is_volatile(&self) -> bool {
        self.modifiers & 0b100000000000!= 0
    }

    pub fn gen_type_modifiers(&self) -> String {
        let modifiers = [
            (self.is_public(), "public"),
            (self.is_private(), "private"),
            (self.is_protected(), "protected"),
            (self.is_static(), "static"),
            (self.is_final(), "final"),
            (self.is_abstract(), "abstract"),
            (self.is_strictfp(), "strictfp"),
        ];
        let modifier_strs :Vec<String>= modifiers
            .iter()
            .filter(|&(is_present, _)| *is_present)
            .map(|(_, modifier)| modifier.to_string())
            .collect();

        modifier_strs.join(" ")    
    }

    pub fn gen_field_modifiers(&self) -> String {
        let modifiers = [
            (self.is_public(), "public"),
            (self.is_private(), "private"),
            (self.is_protected(), "protected"),
            (self.is_static(), "static"),
            (self.is_final(), "final"),
            (self.is_transient(), "transient"),
            (self.is_volatile(), "volatile"),
        ];
        let modifier_strs :Vec<String>= modifiers
            .iter()
            .filter(|&(is_present, _)| *is_present)
            .map(|(_, modifier)| modifier.to_string())
            .collect();

        modifier_strs.join(" ")    
    }

    pub fn gen_method_modifiers(&self) -> String {
        let modifiers = [
            (self.is_public(), "public"),
            (self.is_private(), "private"),
            (self.is_protected(), "protected"),
            (self.is_static(), "static"),
            (self.is_final(), "final"),
            (self.is_abstract(), "abstract"),
            (self.is_strictfp(), "strictfp"),
            (self.is_synchronized(), "synchronized"),
            (self.is_native(), "native"),
        ];
        let modifier_strs :Vec<String>= modifiers
            .iter()
            .filter(|&(is_present, _)| *is_present)
            .map(|(_, modifier)| modifier.to_string())
            .collect();

        modifier_strs.join(" ")    
    }

}

pub enum ValueType {
    Class,
    Value,
}

impl Default for ValueType {
    fn default() -> ValueType {
        ValueType::Value
    }
}

pub struct JavaLanguage {
    pub name: String,
    pub version: String,
    pub file_extension: String
}

impl Default for JavaLanguage {
    fn default() -> JavaLanguage {
        JavaLanguage {
            name: "java".to_string(),
            version: "1.8".to_string(),
            file_extension: "java".to_string()
        }
    }
}

pub struct JavaAnnotationDeclaration{
    pub name: String,
    pub attributes: Vec<JavaAnnotationAttribute>
}

impl JavaAnnotationDeclaration {
    pub fn new(name: &str) -> JavaAnnotationDeclaration {
        JavaAnnotationDeclaration {
            name: name.to_owned(),
            attributes: vec![]
        }
    }

    pub fn add_attribute(&mut self, attribute: JavaAnnotationAttribute) {
        self.attributes.push(attribute);
    }

    pub fn determine_imports(&self) -> Vec<String> {
        let mut imports = vec![];
        if is_import_type(&self.name) {
            imports.push(self.name.clone());
        }
        
        for attribute in &self.attributes {
            let imports_from_attribute = attribute.determine_imports();
            imports.extend(imports_from_attribute);
        }

        imports
    }
}


pub struct JavaAnnotationAttribute {
    pub name: String,
    pub value_type: ValueType,
    pub value: Vec<String>
}

impl JavaAnnotationAttribute {
    pub fn new(name: &str,value_type: ValueType, value: Vec<&str>) -> JavaAnnotationAttribute {
        JavaAnnotationAttribute {
            name:name.to_owned(),
            value_type,
            value: value.iter().map(|&s| s.to_owned()).collect()
        }
    }

    fn determine_imports(&self) -> Vec<String> {
        let mut imports = vec![];
        match self.value_type {
            ValueType::Class => {
                for value in self.value.iter() {
                    if is_import_type(&value) {
                        imports.push(value.clone());
                    }
                }
            },
            _ => {}
        }
        imports
    }
}


pub struct JavaFieldDeclaration{
    pub name: String,
    pub return_type: String,
    pub modifiers: Modifier,
    pub value: Option<String>,
    pub annotations: Vec<JavaAnnotationDeclaration>,
}

impl JavaFieldDeclaration {
    pub fn new(name: &str, return_type: &str, modifiers: u16, value: Option<&str>) -> JavaFieldDeclaration {
        JavaFieldDeclaration {
            name:name.to_owned(),
            return_type:return_type.to_owned(),
            modifiers:Modifier::new(modifiers),
            value:value.map(|s| s.to_owned()),
            annotations: vec![]
        }
    }

    pub fn add_annotation(&mut self, annotation: JavaAnnotationDeclaration) {
        self.annotations.push(annotation);
    }

    pub fn determine_imports(&self) -> Vec<String> {
        let mut imports = vec![];
        if is_import_type(self.return_type.as_str()) {
            imports.push(self.return_type.clone());
        }
        for annotation in &self.annotations {
            for import in annotation.determine_imports() {
                imports.push(import);
            }
        }
        return imports;
    }
}

pub struct JavaMethodParameter{
    pub name: String,
    pub param_type: String,
    pub annotations: Vec<JavaAnnotationDeclaration>
}

impl JavaMethodParameter {
    pub fn new(name: &str, param_type: &str) -> JavaMethodParameter {
        JavaMethodParameter {
            name:name.to_owned(),
            param_type:param_type.to_owned(),
            annotations: vec![]
        }
    }

    pub fn add_annotation(&mut self, annotation: JavaAnnotationDeclaration) {
        self.annotations.push(annotation);
    }

    pub fn determine_imports(&self) -> Vec<String> {
        let mut imports = vec![];
        if is_import_type(self.param_type.as_str()) {
            imports.push(self.param_type.clone());
        }
        for annotation in &self.annotations {
            for import in annotation.determine_imports() {
                imports.push(import);
            }
        }
        return imports;
    }
}

pub struct JavaMethodDeclaration{
    pub name: String,
    pub return_type: String,
    pub modifiers: Modifier, 
    pub annotations: Vec<JavaAnnotationDeclaration>,
    pub parameters: Vec<JavaMethodParameter>,
    pub statements: Vec<JavaMethodStatement>
}

impl JavaMethodDeclaration{
    pub fn new(name: &str, return_type: &str, modifiers: u16) -> JavaMethodDeclaration {
        JavaMethodDeclaration{
            name:name.to_string(),
            return_type:return_type.to_owned(),
            modifiers:Modifier::new(modifiers),
            annotations: vec![],
            parameters: vec![],
            statements: vec![]
        }
    }

    pub fn add_parameter(&mut self, parameter: JavaMethodParameter) {
        self.parameters.push(parameter);
    }

    pub fn add_annotation(&mut self, annotation: JavaAnnotationDeclaration) {
        self.annotations.push(annotation);
    }

    pub fn add_statement(&mut self, statement: JavaMethodStatement) {
        self.statements.push(statement);
    }

    pub fn determine_imports(&self) -> Vec<String> {
        let mut imports = vec![];
        if is_import_type(self.return_type.as_str()) {
            imports.push(self.return_type.clone());
        }
        for annotation in &self.annotations {
            for import in annotation.determine_imports() {
                imports.push(import);
            }
        }
        for parameter in &self.parameters {
            for import in parameter.determine_imports() {
                imports.push(import);
            }
        }

        for statement in &self.statements {
            for import in statement.determine_imports() {
                imports.push(import);
            }
        }
        return imports;
        
    }

}

pub struct JavaMethodStatement {
    pub statement: String,
    pub args: Vec<String>,
}

impl JavaMethodStatement {
    pub fn new(statement: &str,args:Vec<&str>) -> JavaMethodStatement {
        let statement = statement.trim_end_matches('$');
        JavaMethodStatement{
            statement: statement.to_owned(),
            args: args.iter().map(|&s| s.to_owned()).collect(),
        }
    }

    fn determine_imports(&self) -> Vec<String> {
        self.args
            .iter()
            .filter(|&s| is_import_type(s))
            .map(|s| s.to_owned())
            .collect()
        
    }
     
}

/// java class
pub struct JavaTypeDeclaration{
    pub modifiers: Modifier,
    pub name: String,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub fields: Vec<JavaFieldDeclaration>,
    pub methods: Vec<JavaMethodDeclaration>,
    pub annotations: Vec<JavaAnnotationDeclaration>,
}

impl JavaTypeDeclaration {
    pub fn new(modifiers: u16, name: &str, extends: Option<&str>) -> JavaTypeDeclaration {
        JavaTypeDeclaration {
            modifiers: Modifier::new(modifiers),
            name: name.to_owned(),
            extends: extends.map(|s| s.to_owned()),
            implements: vec![],
            fields: vec![],
            methods: vec![],
            annotations: vec![],
        }
    }

    pub fn add_field(&mut self, field: JavaFieldDeclaration) {
        self.fields.push(field);
    }

    pub fn add_method(&mut self, method: JavaMethodDeclaration) {
        self.methods.push(method);
    }

    pub fn add_annotation(&mut self, annotation: JavaAnnotationDeclaration) {
        self.annotations.push(annotation);
    }

    pub fn add_implement(&mut self, implement: String) {
        self.implements.push(implement);
    }

}


/// 最小编译单元,等同于一个.java文件
pub struct JavaCompilationUnit {
    pub package_name: String,
    pub name: String,
    pub type_declarations: Vec<JavaTypeDeclaration>,
}

impl JavaCompilationUnit {
    pub fn new(package_name: &str, name: &str) -> JavaCompilationUnit {
        JavaCompilationUnit {
            package_name:package_name.to_owned(),
            name:name.to_owned(),
            type_declarations: vec![],

        }
    }

    pub fn add_type_declaration(&mut self, type_declaration: JavaTypeDeclaration) {
        self.type_declarations.push(type_declaration);
    }

    pub fn determine_imports(&self) -> Vec<String> {
        let mut imports: Vec<String> = vec![];
        let types = &self.type_declarations;
        types.iter().for_each(|type_declaration| {
            if let Some(extend) = type_declaration.extends.as_ref() {
                imports.push(extend.clone());
            }

            type_declaration.implements.iter().for_each(|i| {
                imports.push(i.clone());
            });

            type_declaration.fields.iter().for_each(|field| {
                imports.extend(field.determine_imports());
            });

            type_declaration.methods.iter().for_each(|method| {
                imports.extend(method.determine_imports());
            });
        });
        return imports;
    }
    
}

pub struct JavaSourceCode {
    pub compilation_units: Vec<JavaCompilationUnit>,
}

impl JavaSourceCode {
    pub fn new() -> JavaSourceCode {
        JavaSourceCode {
            compilation_units: vec![],
        }
    }

    pub fn add_compilation_unit(&mut self, compilation_unit: JavaCompilationUnit) {
        self.compilation_units.push(compilation_unit);
    }
    
}

pub struct JavaSourceStructure{
    pub root_directory: PathBuf,
    pub resource_directory: PathBuf,
    pub source_directory: PathBuf,
    pub file_extension: String,
}

impl JavaSourceStructure{
    pub fn new(root_directory: PathBuf) -> JavaSourceStructure {
        let language = JavaLanguage::default();
        let source_directory = root_directory.join(language.name);
        let resource_directory = root_directory.join("resource");
        JavaSourceStructure {
            root_directory,
            resource_directory,  
            source_directory,
            file_extension: language.file_extension,
        }
    }

    pub fn create_source_file(&self,package_name:String,file_name:String) -> Result<File> {
        let package_directory = package_name.replace(".", "/");
        let file_name = file_name + "." + &self.file_extension;
        if !self.source_directory.join(package_directory.clone()).exists() {
            fs::create_dir_all(self.source_directory.join(package_directory.clone()))?;
        }
        let to = self.source_directory.join(package_directory).join(file_name);
        OpenOptions::new()
           .write(true)
           .create(true)
           .open(to.clone())
           .map_err(|e| e.into())
    }
}



fn is_import_type(ty: &str) -> bool{
    ty.contains(".") && !ty.starts_with("java.lang.")
}

