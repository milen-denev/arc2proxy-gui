use std::{env, fs, io::{Seek, Write}, path::PathBuf};

use models::ProxyConfiguration;

pub mod models;

fn _get_config_path() -> Option<PathBuf> {
    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.push("proxy_config.toml");
    if dir.is_file() {
        Some(dir)
    }
    else {
        None
    }
}

#[tauri::command]
fn get_configuration() -> ProxyConfiguration {
    if let Some(path_buf) = _get_config_path() {
        let toml_string = fs::read_to_string(path_buf).unwrap();
        let model = toml::from_str::<ProxyConfiguration>(&toml_string).unwrap();
        model
    } else {
        panic!("proxy_config.toml File was not found.")
    }
}

#[tauri::command]
fn save_value(setting_name: &str, setting_value: &str) -> bool {
    if let Some(path_buf) = _get_config_path() {
        let toml_string = fs::read_to_string(path_buf).unwrap();
        let mut model = toml::from_str::<ProxyConfiguration>(&toml_string).unwrap();

        if setting_name == "listening_address" {
            model.listening_address = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.into())
                }
            };
        } else if setting_name == "listening_port_http" {
            model.listening_port_http = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<u16>().unwrap())
                }
            };
        } else if setting_name == "listening_port_https" {
            model.listening_port_https = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<u16>().unwrap())
                }
            };
        } else if setting_name == "logging_level" {
            model.logging_level = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.into())
                }
            };
        } else if setting_name == "add_caching" {
            model.add_caching = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<bool>().unwrap())
                }
            };
        }  else if setting_name == "add_rate_limiting" {
            model.add_rate_limiting = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<bool>().unwrap())
                }
            };
        } else if setting_name == "add_logging" {
            model.add_logging = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<bool>().unwrap())
                }
            };
        } else if setting_name == "disable_default_body_limit" {
            model.disable_default_body_limit = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<bool>().unwrap())
                }
            };
        } else if setting_name == "add_sql_injection_protection" {
            model.add_sql_injection_protection = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<bool>().unwrap())
                }
            };
        } else if setting_name == "recv_buffer_size" {
            model.recv_buffer_size = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<usize>().unwrap())
                }
            };
        } else if setting_name == "send_buffer_size" {
            model.send_buffer_size = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<usize>().unwrap())
                }
            };
        } else if setting_name == "ip_ttl" {
            model.ip_ttl = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<u32>().unwrap())
                }
            };
        } else if setting_name == "tcp_keep_alive_seconds" {
            model.tcp_keep_alive_seconds = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<u64>().unwrap())
                }
            };
        } else if setting_name == "max_backlog" {
            model.max_backlog = {
                if setting_value.len() == 0 {
                    None
                } else {
                    Some(setting_value.parse::<i32>().unwrap())
                }
            };
        }

        let new_string = toml::to_string(&model).unwrap();

        let mut toml_file = fs::OpenOptions::new().write(true).truncate(true).open(_get_config_path().unwrap()).unwrap();

        toml_file.set_len(0).unwrap();
        toml_file.rewind().unwrap();
        toml_file.write_all(new_string.as_bytes()).expect("Failed to write");

        true
    } else {
        false
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_configuration, save_value])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
