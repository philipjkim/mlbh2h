const TOML: &'static str = include_str!("../Cargo.toml");

pub fn get() -> &'static str {
    if let Some(line) = TOML.split("\n").find(|x| x.starts_with("version")) {
        let start = line.find('"').unwrap() + 1;
        let end = line.rfind('"').unwrap();
        &line[start..end]
    } else {
        panic!("failed parsing version from Cagro.toml");
    }
}
