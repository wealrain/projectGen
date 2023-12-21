use std::{
    path::PathBuf, 
    fs::{File, OpenOptions, self}, 
    io::Write,  fmt, 
};

use crate::Result;

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

pub struct JavaAnnotationDeclaration{}
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
}

pub struct JavaMethodDeclaration{
    pub name: String,
    pub return_type: String,
    pub modifiers: Modifier, 
    pub annotations: Vec<JavaAnnotationDeclaration>,
    pub parameters: Vec<JavaMethodParameter>,
}

impl JavaMethodDeclaration{
    pub fn new(name: &str, return_type: &str, modifiers: u16) -> JavaMethodDeclaration {
        JavaMethodDeclaration{
            name:name.to_string(),
            return_type:return_type.to_owned(),
            modifiers:Modifier::new(modifiers),
            annotations: vec![],
            parameters: vec![],
        }
    }

    pub fn add_parameter(&mut self, parameter: JavaMethodParameter) {
        self.parameters.push(parameter);
    }

    pub fn add_annotation(&mut self, annotation: JavaAnnotationDeclaration) {
        self.annotations.push(annotation);
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
        vec![]
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

#[derive(Debug,Clone)]
pub struct JavaSourceCodeWriter{
    level: u8,
    ident: String,
    need_ident: bool,
}

impl JavaSourceCodeWriter{
    pub fn new() -> JavaSourceCodeWriter{
        JavaSourceCodeWriter{
            level: 0,
            ident: "    ".to_string(),
            need_ident: false,
        }
    }

    pub fn write(&mut self,structure: &JavaSourceStructure,source_code:JavaSourceCode) -> Result<()> {
        let compilation_units = source_code.compilation_units;
        for compilation_unit in compilation_units {
            self.write_compilation_unit(structure, compilation_unit)?;
        }

        Ok(())
    }

    pub fn write_to(&mut self,file:&mut File,fmt: fmt::Arguments<'_>) -> Result<()> {
        if self.need_ident {
            write!(file,"{}",self.ident.repeat(self.level as usize))?;
            self.need_ident = false;
        }
        file.write_fmt(fmt)?;
        Ok(())
    }


    fn need_ident(&mut self) {
        self.need_ident = true;
    }

    fn write_with_indent<O:FnOnce(&mut File,&mut JavaSourceCodeWriter) -> Result<()>>(&mut self,file:&mut File,mut op:O) -> Result<()> {
        self.level += 1;
        op(file,self)?;
        self.level -= 1;
        Ok(())
    }

    fn write_compilation_unit(&mut self,structure: &JavaSourceStructure,compilation_unit: JavaCompilationUnit) -> Result<()> {
        let mut file = structure.create_source_file(compilation_unit.package_name.clone(), compilation_unit.name)?;
        self.write_to(&mut file, format_args!("package {};\n\n",compilation_unit.package_name))?;
        // todo write import
        // todo write comment
        let type_declarations =  compilation_unit.type_declarations;
        for type_declaration in type_declarations {
            // todo write annotations
            let modifers_str = type_declaration.modifiers.gen_type_modifiers();
            self.write_to(&mut file, format_args!("{} class {} ",modifers_str,type_declaration.name))?;
            // todo write extends
            // todo write implements
            self.write_to(&mut file, format_args!(" {{\n\n"))?;
            self.need_ident();
            if type_declaration.fields.len() > 0 {
                self.write_type_fields(&mut file,type_declaration.fields)?;
            }
            
            if type_declaration.methods.len() > 0 {
                self.write_type_methods(&mut file,type_declaration.methods)?;
            }

            self.write_to(&mut file, format_args!("\n}}"))?;
        }
        
        
        Ok(())
    }

    fn write_type_fields(&mut self,file:&mut File,field_declarations:Vec<JavaFieldDeclaration>)->Result<()> {
        
        self.write_with_indent(file,|file,writer|{
            for field_declaration in field_declarations {
                // todo write annotation
                writer.need_ident(); 
                let modfier_str = field_declaration.modifiers.gen_field_modifiers();
                writer.write_to(file, format_args!("{} ",modfier_str))?;
                writer.write_to(file, format_args!("{} ",get_unqualified_name(field_declaration.return_type)))?;
                writer.write_to(file, format_args!("{}",field_declaration.name))?;
                if let Some(value) = field_declaration.value {
                    writer.write_to(file, format_args!(" = {}",value))?;
                }
                writer.write_to(file, format_args!(";\n\n"))?;
                
            }
            
            Ok(())
        })?;

        Ok(())
    }

    fn write_type_methods(&mut self,file:&mut File,method_declarations:Vec<JavaMethodDeclaration>)->Result<()> {
        
        self.write_with_indent(file,|file,writer|{
            for method_declaration in method_declarations {
                // todo write annotations
                writer.need_ident();
                let modfier_str = method_declaration.modifiers.gen_method_modifiers();
                writer.write_to(file, format_args!("{} ",modfier_str))?;
                writer.write_to(file, format_args!("{} {}(",get_unqualified_name(method_declaration.return_type),method_declaration.name))?;
                let params = method_declaration.parameters;
                if params.len() > 0 {
                    writer.write_method_paramters(file, params)?;
                }
                writer.write_to(file, format_args!(") {{\n"))?;
                // todo write codeblock
                
                // method over
                writer.write_to(file, format_args!("\n"))?;
                writer.need_ident();
                writer.write_to(file, format_args!("}}\n"))?;
            }
            Ok(())   
        })?;
        
        

        Ok(())
    }

    fn write_method_paramters(&mut self,file:&mut File,parameters:Vec<JavaMethodParameter>) -> Result<()> {
        for (i,param) in parameters.iter().enumerate() {
            if i>0 {
                self.write_to(file, format_args!(","))?;
            }
             // todo write annotations
            self.write_to(file, format_args!("{} {}",get_unqualified_name(param.param_type.to_string()),param.name))?;
        }
        
        Ok(())
    }


}

fn get_unqualified_name(name: String) -> String{
    if !name.contains(".") {
        return name;
    }

    name.rsplit(".").next().to_owned().unwrap().to_string()
}

