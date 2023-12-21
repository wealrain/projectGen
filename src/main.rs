use project_gen::*;
fn main() {
    let parser = Parser::parse("D:/rustwrokspace/projectGen/demo.yaml").unwrap();
    println!("{:?}", parser)
}
 
