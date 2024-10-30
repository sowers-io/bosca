use std::io::{Error, ErrorKind};
use yaml_rust2::{ScanError, Yaml, YamlLoader};

pub fn parse_string(string: &str) -> Result<Yaml, Error> {
    let y: Result<Vec<Yaml>, ScanError> = YamlLoader::load_from_str(string);
    match y {
        Ok(yaml) => {
            if yaml.is_empty() {
                Err(ErrorKind::NotFound.into())
            } else {
                Ok(yaml.first().unwrap().clone())
            }
        }
        Err(_) => Err(ErrorKind::Other.into()),
    }
}
