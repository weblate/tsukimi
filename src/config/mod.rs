use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::Read};
use toml;
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct Config {
    pub domain: String,
    pub username: String,
    pub password: String,
    pub port: String,
    pub user_id: String,
    pub access_token: String,
    pub proxy: String,
}

fn generate_uuid() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

pub fn load_cfg() {
    let path = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("config")
        .join("tsukimi.toml");

    if path.exists() {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();
        env::set_var("EMBY_DOMAIN", &config.domain);
        env::set_var("EMBY_USERNAME", &config.username);
        env::set_var("EMBY_PASSWORD", &config.password);
        env::set_var("EMBY_PORT", &config.port);
        env::set_var("EMBY_USER_ID", &config.user_id);
        env::set_var("EMBY_ACCESS_TOKEN", &config.access_token);
        env::set_var("EMBY_PROXY", &config.proxy);

        let uuid = generate_uuid();
        env::set_var("UUID", &uuid);

        let mpv_config_file = env::current_dir().unwrap().parent().unwrap().join("mpv");
        let mpv_config = if mpv_config_file.exists() {
            "true"
        } else {
            "false"
        };
        env::set_var("MPV_CONFIG", mpv_config);
        env::set_var("MPV_CONFIG_DIR", mpv_config_file.display().to_string());
    } else {
        let uuid = generate_uuid();
        env::set_var("UUID", &uuid);

        let mpv_config_file = env::current_dir().unwrap().parent().unwrap().join("mpv");
        let mpv_config = if mpv_config_file.exists() {
            "true"
        } else {
            "false"
        };
        env::set_var("MPV_CONFIG", mpv_config);
        env::set_var("MPV_CONFIG_DIR", mpv_config_file.display().to_string());
    };
}

pub fn set_config() -> Config {
    let config = Config {
        domain: env::var("EMBY_DOMAIN").unwrap(),
        username: env::var("EMBY_USERNAME").unwrap(),
        password: env::var("EMBY_PASSWORD").unwrap(),
        port: env::var("EMBY_PORT").unwrap(),
        user_id: env::var("EMBY_USER_ID").unwrap(),
        access_token: env::var("EMBY_ACCESS_TOKEN").unwrap(),
        proxy: env::var("EMBY_PROXY").unwrap(),
    };
    config
}

pub fn get_device_name() -> String {
    if cfg!(target_os = "windows") {
        env::var("COMPUTERNAME").unwrap_or("Unknown Device".to_string())
    } else {
        env::var("HOSTNAME").unwrap_or("Unknown Device".to_string())
    }
}
