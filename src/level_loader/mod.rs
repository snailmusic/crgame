use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
use level::Level;
use thiserror::Error;
pub mod level;

#[derive(Default)]
pub struct LevelLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LevelLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not parse yaml: {0}")]
    YamlError(#[from] serde_yaml::Error)
}


impl AssetLoader for LevelLoader {
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Level, LevelLoaderError> {
        info!("Loading Level...");
        let mut buffer: String = "".to_string();
        reader.read_to_string(&mut buffer).await?;
        let asset: Level = serde_yaml::from_str::<Level>(&buffer)?;

        Ok(
            asset
        )
    }

    fn extensions(&self) -> &[&str] {
        &[".yml", ".yaml"]
    }
    
    type Asset = Level;
    
    type Settings = ();
    
    type Error = LevelLoaderError;

}