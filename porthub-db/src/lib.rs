use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

pub mod errors;
pub mod manifest;
pub mod storage;
pub mod models;

pub use errors::Result;
pub use manifest::Manifest;
pub use models::{Link, Tag};

pub struct PorthubDB {
    pub data_dir: PathBuf,
    pub manifest: RwLock<Manifest>,
}

impl PorthubDB {
    pub fn init(data_dir: PathBuf) -> Self {
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)
                .map_err(|e| crate::errors::Error::Io(e))?;
        }

        let manifest = Manifest::default();

        Ok(Self {
            data_dir,
            manifest: RwLock::new(manifest),
        })
    }

    fn create_manifest(&self) -> Manifest {
        fs::File::create(self.data_dir.join("manifest.json"))?;
    }

    pub fn get_manifest(&self) -> Manifest {
        manifest::Manifest { spaces: vec![] }
    }

    fn update_manifest(&self, manifest: Manifest) {
        todo!()
    }

    pub fn create_space(&self, space_name: &str) -> Result<()> {
        let mut manifest_guard = self.manifest.write().unwrap();

        let space_dir = self.data_dir.join(space_name);
        fs::create_dir(&space_dir)
            .map_err(|e| crate::errors::Error::Io(e)) ?; // ErrorKind::AlreadyExists

        manifest_guard.spaces.insert(space_name.to_string(), Vec::new());

        let manifest_path = self.data_dir.join("manifest.json");
        let content = serde_json::to_string_pretty(&*manifest_guard)
            .map_err(|e| crate::errors::Error::Json(e))?;

        fs::write(manifest_path, content)
            .map_err(|e| crate::errors::Error::Io(e))?;

        Ok(())
    }

    pub fn delete_space(&self, space_name: &str) -> Result<()> {
        fs::remove_dir_all(self.data_dir.join(space_name))?;
        todo!()
    }

    pub fn update_space(&self, space_name: &str, new_space_name: &str) -> Result<()> {
        let old_path = self.data_dir.join(space_name);
        let new_path = self.data_dir.join(new_space_name);

        fs::rename(old_path, new_path)
            .map_err(|e| crate::errors::Error::Io(e))?;

        todo!();
        // write lok to manifest
        // rename key in HashMap
        // update file manifest.json

        Ok(())
    }

    pub fn create_group(&self, in_space: &str, group_name: &str) -> Result<()> {
        todo!()
    }

    pub fn delete_group(&self, group_name: &str) -> Result<()> {
        todo!()
    }

    pub fn update_group(&self, in_space: &str, group_name: &str, new_group_name: &str) -> Result<()> {
        todo!()
    }

    pub fn get_links(&self, space: &str, group_name: &str) -> Vec<Link> {
        todo!()
    }

    pub fn add_link(&self, space: &str, group_name: &str, link: models::Link) -> Result<()> {
        todo!()
    }

    pub fn delete_link(&self, space: &str, group_name: &str, link_id: int) -> Result<()> {
        todo!()
    }

    pub fn update_link(&self, space: &str, group: &str, link_id: &str, updated_link: Link) -> crate::errors::Result<()> {
        todo!()
    }
}
