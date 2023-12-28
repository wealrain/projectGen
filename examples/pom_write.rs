use project_gen::java::{POMWriter, POM, Property};

fn main() {
    let mut pom_writer = POMWriter::default();
    let mut pom = POM::new("cn.ljyun", "test");
    pom.add_property(Property::new("maven.compiler.source","8"));
    pom.add_property(Property::new("maven.compiler.target","8"));
    let root_dir = std::env::current_dir().unwrap();
    let dir = root_dir.join("java");
    pom_writer.write_to(&pom,dir);
}