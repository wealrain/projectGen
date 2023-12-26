use std::{
    fs::File, 
    io::Write,  
    fmt, 
    collections::{HashSet, HashMap}, 
};

use crate::Result;

use super::{
    JavaSourceStructure, 
    JavaSourceCode, 
    JavaCompilationUnit, 
    JavaFieldDeclaration, 
    JavaMethodDeclaration, JavaMethodParameter,JavaMethodStatement, JavaAnnotationDeclaration, ValueType};



#[derive(Debug,Clone)]
pub struct JavaSourceCodeWriter{    
}

impl JavaSourceCodeWriter{
    pub fn new() -> JavaSourceCodeWriter{
        JavaSourceCodeWriter{}
    }

    pub fn write(&mut self,structure: &JavaSourceStructure,source_code:JavaSourceCode) -> Result<()> {
        let compilation_units = source_code.compilation_units;
        for compilation_unit in &compilation_units {
            // todo 后续使用异步写文件
            let mut compilation_unit_writer = CompilationUnitWriter::new();
            compilation_unit_writer.write_compilation_unit(structure, compilation_unit)?;

        }

        Ok(())
    }

    

}

struct CompilationUnitWriter {
    level: u8,
    ident: String,
    need_ident: bool,
    imports: HashSet<String>
}

impl CompilationUnitWriter {
    fn new() -> CompilationUnitWriter{
        CompilationUnitWriter{
            level: 0,
            ident: "    ".to_string(),
            need_ident: false,
            imports: HashSet::new()
        }
    }

    fn write_to(&mut self,file:&mut File,fmt: fmt::Arguments<'_>) -> Result<()> {
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

    fn write_with_indent<O:FnOnce(&mut File,&mut CompilationUnitWriter) -> Result<()>>(&mut self,file:&mut File,mut op:O) -> Result<()> {
        self.level += 1;
        op(file,self)?;
        self.level -= 1;
        Ok(())
    }

    fn write_compilation_unit(&mut self,structure: &JavaSourceStructure,compilation_unit: &JavaCompilationUnit) -> Result<()> {
        let mut file = structure.create_source_file(compilation_unit.package_name.clone(), compilation_unit.name.clone())?;
        // write package
        self.write_to(&mut file, format_args!("package {};\n\n",compilation_unit.package_name))?;
        // write imports
        self.write_imports(&mut file,compilation_unit.determine_imports())?;
        self.write_to(&mut file, format_args!("\n"))?;
        // write class
        let type_declarations =  &compilation_unit.type_declarations;
        for type_declaration in type_declarations {
            self.write_annotation(&mut file, &type_declaration.annotations, false, true)?;
            let modifers_str = type_declaration.modifiers.gen_type_modifiers();
            self.write_to(&mut file, format_args!("{} class {} ",modifers_str,type_declaration.name))?;
            // todo write extends
            if let Some(extend) = type_declaration.extends.as_ref() {
                self.write_to(&mut file, format_args!("extends {} ",extend))?;
            }
            // todo write implements
            self.write_to(&mut file, format_args!(" {{\n\n"))?;
            self.need_ident();
            if type_declaration.fields.len() > 0 {
                self.write_type_fields(&mut file,&type_declaration.fields)?;
            }
            
            if type_declaration.methods.len() > 0 {
                self.write_type_methods(&mut file,&type_declaration.methods)?;
            }

            self.write_to(&mut file, format_args!("\n}}"))?;
        }
        
        
        Ok(())
    }

    fn write_imports(&mut self,file:&mut File,imports:Vec<String>)->Result<()> {
        let mut class_map = HashMap::<String,String>::new();
        imports.iter().for_each(|x|{
            class_map.insert(self.get_unqualified_name(x.clone()),x.clone());
        });
        let imports:HashSet<String> = class_map.values().cloned().collect();
        self.imports = imports.clone();
        for import in  imports {
            self.write_to(file, format_args!("import {};\n",import))?;
        }

        Ok(())
    }

    fn write_type_fields(&mut self,file:&mut File,field_declarations:&Vec<JavaFieldDeclaration>)->Result<()> {
        
        self.write_with_indent(file,|file,writer|{
            for field_declaration in field_declarations {
                writer.write_annotation(file,&field_declaration.annotations,true,true)?;
                writer.need_ident(); 
                let modfier_str = field_declaration.modifiers.gen_field_modifiers();
                writer.write_to(file, format_args!("{} ",modfier_str))?;
                writer.write_to(file, format_args!("{} ",writer.get_unqualified_name(field_declaration.return_type.clone())))?;
                writer.write_to(file, format_args!("{}",field_declaration.name))?;
                if let Some(value) = &field_declaration.value {
                    writer.write_to(file, format_args!(" = {}",value))?;
                }
                writer.write_to(file, format_args!(";\n\n"))?;
                
            }
            
            Ok(())
        })?;

        Ok(())
    }

    fn write_annotation(&mut self,file:&mut File,annotations:&Vec<JavaAnnotationDeclaration>,need_indent:bool,need_wrap:bool)->Result<()> {
        for annotation in annotations {
            if need_indent {
                self.need_ident();
            }
            self.write_to(file, format_args!("@{}",annotation.name))?;
            if !annotation.attributes.is_empty() {
                self.write_to(file, format_args!("("))?;
            }
            let mut need_comma = false;
            for attribute in &annotation.attributes {
                if need_comma {
                    self.write_to(file, format_args!(","))?;
                } else {
                    need_comma = true;
                }
                if attribute.name == "value" {
                    self.write_to(file, format_args!("\"{}\"", attribute.value.join(",")))?;
                    continue;
                }
    
                let mut attrs: Vec<String> = vec![];
                match attribute.value_type{
                    ValueType::Class => {
                        // attrs.push(attribute.value.join("."));
                    },
                    _ =>{}
                }
    
                self.write_to(file, format_args!( "{}={{{}}}", attribute.name, attribute.value.join(",")))?;
            }
            if !annotation.attributes.is_empty() {
                self.write_to(file, format_args!(")"))?;
            }

            
            if need_wrap {
                self.write_to(file, format_args!("\n"))?;
            }
        }
        Ok(())
    }

    fn write_type_methods(&mut self,file:&mut File,method_declarations:&Vec<JavaMethodDeclaration>)->Result<()> {
        
        self.write_with_indent(file,|file,writer|{
            for method_declaration in method_declarations {
                writer.write_annotation(file,&method_declaration.annotations,true,true)?;
                writer.need_ident();
                let modfier_str = method_declaration.modifiers.gen_method_modifiers();
                writer.write_to(file, format_args!("{} ",modfier_str))?;
                writer.write_to(file, format_args!("{} {}(",writer.get_unqualified_name(method_declaration.return_type.clone()),method_declaration.name))?;
                let params = &method_declaration.parameters;
                if params.len() > 0 {
                    writer.write_method_paramters(file, params)?;
                }
                writer.write_to(file, format_args!(") {{\n"))?;
                writer.write_with_indent(file,|file,writer|{
                    writer.write_method_statements(file,&method_declaration.statements)?;
                    Ok(())  
                })?;
                // method over
                writer.write_to(file, format_args!("\n"))?;
                writer.need_ident();
                writer.write_to(file, format_args!("}}\n"))?;
            }
            Ok(())   
        })?;
        
        

        Ok(())
    }

    fn write_method_statements(&mut self,file:&mut File,statements:&Vec<JavaMethodStatement>) -> Result<()> {
        for statement in statements {
            self.need_ident();
            let args = &statement.args;
            let statement = &statement.statement;
            let mut result = String::from(statement);
            let mut arg_index = 0;
            for p in 0..statement.len() {
                if statement.chars().nth(p).unwrap() == '$' {
                    let param_name = statement.chars().nth(p+1).unwrap();
                    if param_name == 'T' {
                        let replace_value = self.get_unqualified_name(args[arg_index].clone());
                        result = result.replacen("$T", &replace_value,1);
                    
                    } else if param_name == 'V' {
                        result = result.replacen("$V", args[arg_index].as_str(),1);
                    }  
                    arg_index += 1;
                }
            }
            self.write_to(file, format_args!("{} ",result))?;
            self.write_to(file, format_args!("\n"))?;
        }
        Ok(())
    }

    fn write_method_paramters(&mut self,file:&mut File,parameters:&Vec<JavaMethodParameter>) -> Result<()> {
        for (i,param) in parameters.iter().enumerate() {
            if i>0 {
                self.write_to(file, format_args!(","))?;
               
            }
            self.write_annotation(file, &param.annotations, false,false)?;
            self.write_to(file, format_args!("{} {}",self.get_unqualified_name(param.param_type.to_string()),param.name))?;
        }
        
        Ok(())
    }

    pub fn get_unqualified_name(&self,name: String) -> String{
        if !name.contains(".") {
            return name;
        }
        
        if name.starts_with("java.lang.") {
            return name.rsplit(".").last().unwrap().to_string();
        }

        if !self.imports.contains(&name) {
            return name;
        }
        name.rsplit(".").next().to_owned().unwrap().to_string()
    }

}