use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub header: HashMap<usize, String>,
    pub data: Vec<HashMap<usize, String>>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            header: HashMap::new(),
            data: vec![],
        }
    }

    pub fn extract_data_from_csv(&self, file: &str) -> Result<Data, String> {
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;

        let file = File::open(file).map_err(|e| e.to_string()).unwrap();
        let mut reader = BufReader::new(file).lines();
        let mut string_buf: Vec<String> = Vec::new();

        while let Some(line) = reader.next() {
            let line = line.unwrap();
            string_buf.push(line);
        }
        let header = string_buf[0].clone();
        let mut doc = string_buf[1..].to_vec();
        let mut doc_indexed: Vec<HashMap<usize, String>> = vec![];

        for full_line in doc.iter_mut() {
            let e = full_line
                .split(",")
                .enumerate()
                .map(|(i, e)| (i, e.to_owned()))
                .collect::<HashMap<usize, String>>();

            doc_indexed.push(e.to_owned());
        }

        let header = header
            .split(",")
            .enumerate()
            .map(|(i, e)| (i, e.to_owned()))
            .collect::<HashMap<usize, String>>();

        let data = Data {
            header,
            data: doc_indexed,
        };
        // println!("{:#?}", data);

        Ok(data)
    }
}
