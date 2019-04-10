use std::fs;

pub fn get() -> &'static str {
    let text = fs::read_to_string("Cargo.toml").unwrap();
    for line in text.split("\n") {
        if line.starts_with("version") {
            let start = line.find('"').unwrap() + 1;
            let end = line.rfind('"').unwrap();
            let version = &line[start..end];
            return Box::leak(version.to_string().into_boxed_str());
        }
    }

    panic!("failed parsing version from Cagro.toml");
}
