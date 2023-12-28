use project_gen::java::{POMWriter, POM};

fn main() {
    let mut pom_writer = POMWriter::default();
    let mut pom = POM::new("cn.ljyun", "test");
    let root_dir = std::env::current_dir().unwrap();
    let dir = root_dir.join("java");
    pom_writer.write_to(&pom,dir);
}