use std::collections::HashSet;

use anyhow::{Context, Result};
use fontdb::{Database, Family, Query, Source};

pub struct FontQuery {
    db: Database,
}

impl FontQuery {
    pub fn new() -> Self {
        let mut db = Database::new();
        db.load_system_fonts();
        Self { db }
    }

    pub fn list_monospaced(&self) -> Vec<String> {
        let mut uniques: HashSet<String> = HashSet::new();
        self.db
            .faces()
            .filter(|face| face.monospaced)
            .filter_map(|face| face.families.first().map(|(name, _)| name.to_string()))
            .for_each(|name| {
                uniques.insert(name);
            });
        uniques.into_iter().collect()
    }

    pub fn load_font(&self, name: Option<&String>) -> Result<Vec<u8>> {
        let query = Query {
            families: &[Family::Name(name.unwrap())],
            ..Default::default()
        };

        self.db
            .query(&query)
            .and_then(|face_id| self.db.face_source(face_id)) // Get Option<(Source, u32)>
            .and_then(|(source, _index)| {
                // Destructure tuple here
                match source {
                    Source::Binary(blob) => {
                        let data: &[u8] = blob.as_ref().as_ref();
                        Some(data.to_vec())
                    }
                    Source::SharedFile(path, _) => std::fs::read(path).ok(),
                    Source::File(path) => std::fs::read(path).ok(),
                }
            })
            .context("Font not found")
    }
}
