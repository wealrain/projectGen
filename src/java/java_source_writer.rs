use std::{
    fs::File, 
    io::Write,  
    fmt, 
    collections::HashSet, 
};

use crate::Result;

use super::{JavaSourceStructure, JavaSourceCode, JavaCompilationUnit, JavaFieldDeclaration, JavaMethodDeclaration, JavaMethodParameter, java_source_code::get_unqualified_name, JavaMethodStatement, JavaAnnotationDeclaration};



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
        for mut compilation_unit in compilation_units {
            self.write_compilation_unit(structure, &mut compilation_unit)?;
        }

        Ok(())
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

    fn write_with_indent<O:FnOnce(&mut File,&mut JavaSourceCodeWriter) -> Result<()>>(&mut self,file:&mut File,mut op:O) -> Result<()> {
        self.level += 1;
        op(file,self)?;
        self.level -= 1;
        Ok(())
    }

    fn write_compilation_unit(&mut self,structure: &JavaSourceStructure,compilation_unit: &mut JavaCompilationUnit) -> Result<()> {
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
            if let Some(extend) = type_declaration.extend {
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
        let unique_vec: Vec<String> = imports.into_iter().collect::<HashSet<_>>().into_iter().collect();
        for import in unique_vec {
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
                writer.write_to(file, format_args!("{} ",get_unqualified_name(field_declaration.return_type.clone())))?;
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
            self.write_to(file, format_args!("{}",annotation))?;
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
                writer.write_to(file, format_args!("{} {}(",get_unqualified_name(method_declaration.return_type.clone()),method_declaration.name))?;
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
            self.write_to(file, format_args!("{} ",statement.statement))?;
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
            self.write_to(file, format_args!("{} {}",get_unqualified_name(param.param_type.to_string()),param.name))?;
        }
        
        Ok(())
    }


}

