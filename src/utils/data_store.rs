use std::{fs::File, path::Path};
use simsearch::SimSearch;
use csv::{ReaderBuilder, WriterBuilder, Writer, StringRecord};
use blake3::Hash;
use hex::encode;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A simple datastore that can persist data on file
///
pub struct DataStore {
    data: HashMap<Vec<u8>, String>, // Use Vec<u8> as the key type
    index: SimSearch<Vec<u8>>, // Use Vec<u8> as the key type
}

impl DataStore {
    /// Initialize an empty datastore
    ///
    pub fn new() -> Self {
        DataStore {
            data: HashMap::new(),
            index: SimSearch::new(),
        }
    }

    /// Load the datastore with the records found
    ///
    pub fn load(&mut self, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);

        for result in rdr.records() {
            let record: StringRecord = result?;
            // Process the record and insert it into the data store
        }

        Ok(())
    }

    /// Persist the datastore to disk, overwriting existing files
    ///
    pub fn save(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let mut wtr = WriterBuilder::new().from_writer(file);
    
        // Placeholder for writing records to the CSV file
        for (key, value) in &self.data {
            wtr.write_record(&[encode(&key), value.to_string()])?;
        }
    
        wtr.flush()?;
        Ok(())
    }

    /// If the record exists, returns the existing one
    ///
    pub fn insert(&mut self, _path: &Path) -> Result<()> {
        // Placeholder for inserting a record into the data store
        Ok(())
    }
}