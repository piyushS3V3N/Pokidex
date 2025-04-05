use crate::configs::Configs;
use crate::{DynResult, Pokidex, PokidexList};
use directories_next::ProjectDirs;
use lazy_static::lazy_static;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Application data and config files path
struct ProjPaths {
    data_path: PathBuf,
    config_path: PathBuf,
}

lazy_static! {
    static ref PROJ_PATHS: ProjPaths = {
        let proj_dirs = ProjectDirs::from("", "", "Pokidex").unwrap();
        fs::create_dir_all(proj_dirs.data_dir()).unwrap();
        fs::create_dir_all(proj_dirs.config_dir()).unwrap();

        let data_path = proj_dirs.data_dir().join("data.json");
        let config_path = proj_dirs.config_dir().join("config.json");

        ProjPaths {
            data_path,
            config_path,
        }
    };
}

/// Load all saved pokientrys from file
pub fn load_pokientrys() -> Result<Vec<Pokidex>, io::Error> {
    if !Path::new(PROJ_PATHS.data_path.as_path()).exists() {
        fs::File::create(PROJ_PATHS.data_path.as_path())?;
    }

    let stringified_pokientrys = fs::read_to_string(PROJ_PATHS.data_path.as_path())?;
    let pokientry_list: PokidexList = serde_json::from_str(&stringified_pokientrys).unwrap_or_default();

    Ok(pokientry_list.pokientrys)
}

/// Save all pokientrys to a file
pub fn save_pokientrys(pokientrys: &[Pokidex]) -> DynResult {
    let pokientrys = &PokidexList::new(pokientrys);
    let stringified_pokientrys = serde_json::to_string(pokientrys)?;
    fs::write(PROJ_PATHS.data_path.as_path(), stringified_pokientrys)?;

    Ok(())
}

/// Load configs from the file and returns it, if there's no config set, returns default config
pub fn load_configs() -> Result<Configs, io::Error> {
    if !Path::new(PROJ_PATHS.config_path.as_path()).exists() {
        save_configs(&Configs::default()).unwrap();
    }

    let stringified_configs = fs::read_to_string(PROJ_PATHS.config_path.as_path())?;
    let configs: Configs = serde_json::from_str(&stringified_configs).unwrap();

    Ok(configs)
}

/// Save configs to file
fn save_configs(configs: &Configs) -> DynResult {
    let stringified_configs = serde_json::to_string_pretty(configs)?;
    fs::write(PROJ_PATHS.config_path.as_path(), stringified_configs)?;

    Ok(())
}
