use project_gen::java::{
    JavaSourceCodeWriter,
    JavaSourceStructure, 
    JavaSourceCode, 
    JavaCompilationUnit, 
    JavaTypeDeclaration, 
    JavaFieldDeclaration, 
    JavaMethodDeclaration, 
    JavaMethodParameter, 
    JavaMethodStatement, 
    JavaAnnotationDeclaration, 
    JavaAnnotationAttribute, 
    ValueType
};
fn main() {
    let root_dir = std::env::current_dir().unwrap();
    let java_structure = JavaSourceStructure::new(root_dir);
    let mut java_source_code = JavaSourceCode::new();
    let mut compilation_unit = JavaCompilationUnit::new("com.example.test","Test");
    let mut type_declaration = JavaTypeDeclaration::new(2, "Test", None);
    
    let mut field1 = JavaFieldDeclaration::new("name","java.lang.String",4,Some("zhangsan"));
    let field2 = JavaFieldDeclaration::new("age","int",4,Some("12"));
    let field3 = JavaFieldDeclaration::new("card","cn.ljyun.entity.Card",4,Some("new Card()"));
    
    let mut annotation = JavaAnnotationDeclaration::new("TestAnnotation");
    annotation.add_attribute(JavaAnnotationAttribute::new("value",ValueType::default(),vec!["test"]));
    annotation.add_attribute(JavaAnnotationAttribute::new("value2",ValueType::Class,vec!["cn.ljyun.Test","com.baidu.Test"]));
    field1.add_annotation(annotation);
    type_declaration.add_field(field1); 
    type_declaration.add_field(field2);
    type_declaration.add_field(field3);

    

    let mut method = JavaMethodDeclaration::new("test", "void", 2);
    let param = JavaMethodParameter::new("card","cn.ljyun.Card");
    method.add_parameter(param);
    let st1 = JavaMethodStatement::new("$T $V = new $T();",vec!["cn.ljyun.Card","card","cn.ljyun.entity.Card"]);
    let st2 = JavaMethodStatement::new("System.out.println($V.getName());",vec!["card"]);
    method.add_statement(st1);
    method.add_statement(st2);
    

    type_declaration.add_method(method);
    compilation_unit.add_type_declaration(type_declaration);
    java_source_code.add_compilation_unit(compilation_unit);

    let mut java_source_writer = JavaSourceCodeWriter::new();
    java_source_writer.write(&java_structure, java_source_code).unwrap();
}