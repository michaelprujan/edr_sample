use std::{error::Error, fs};

pub fn read_json(file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data = fs::read_to_string(file)?;
    let from_json = serde_json::from_str(&data)?;

    Ok(from_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_json() {
        let file_path = r"C:\rust\cybereason\edr\config\1.json";
        let providers = read_json(file_path).unwrap();
        let actual = vec!["7dd42a49-5329-4832-8dfd-43d979153a88"];

        assert_eq!(actual, providers)
    }
}
