use crystal_manifest::parse::CrystalParseErr;
use crystal_manifest::parse_toml;

use std::fs;

const TOML_EXAMPLE1: &str = include_str!("tomls/example.toml");
const TOML_PARSE_ERR: &str = "tests/tomls/parse_err";

/// Test that all TOML files in `parse_err` directory are invalid
#[test]
fn fail() {
    let files = fs::read_dir(TOML_PARSE_ERR).unwrap().count();
    for file in fs::read_dir(TOML_PARSE_ERR).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        let toml = fs::read_to_string(path).unwrap();
        let parsed = parse_toml(&toml);
        assert!(parsed.is_err());
    }
    println!("{} files, test completed", files);
}

#[test]
fn bless_example1() -> Result<(), CrystalParseErr> {
    let parsed = parse_toml(TOML_EXAMPLE1)?;
    println!("{:#?}", parsed);
    Ok(())
}
