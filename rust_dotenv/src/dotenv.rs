use std::collections::HashMap;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::env;

use crate::vars::parse_var;

#[derive(Debug, Clone)]
pub struct DotEnv {
    vars: HashMap<String, String>
}

impl DotEnv {

    pub fn new(env: &str) -> DotEnv {
        
        let dotenv_filename = if env.is_empty() {
            ".env".to_string()
        } 
        else {
            format!(".env.{}", env)
        };

        let dotenv_filename_local = format!("{}.local", dotenv_filename);
        let dotenv_filename_to_load = if Path::new(&dotenv_filename_local).exists() {
            &dotenv_filename_local
        } else {
            &dotenv_filename
        };

        let vars = Self::load_env(&dotenv_filename_to_load).unwrap_or_else(|_| {
            eprintln!("Error: {} file not found. No variables were loaded.", dotenv_filename_to_load);
            HashMap::new()
        });

        DotEnv { vars }
    }

    pub fn load_env(filename: &str) -> io::Result<HashMap<String, String>> {

        let mut vars: HashMap<String, String> = HashMap::new();

        let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
        let dotenv_path: PathBuf = Self::find(&current_dir, &filename).unwrap_or_else(|| {
            eprintln!("Error: {} file not found. No variables were loaded.", filename);
            PathBuf::new() // Chemin vide si le fichier n'est pas trouvé
        });

        if let Ok(file) = File::open(dotenv_path) {
            for line in io::BufReader::new(file).lines() {
                if let Ok(line) = line {
                    if !line.starts_with("#") && !line.is_empty() {
                        let parts: Vec<&str> = line.splitn(2, '=').collect();
                        if parts.len() == 2 {

                            let key = parts[0].trim();
                            let value = parts[1].trim();
                            let value_parsed = parse_var(value.to_string());

                            vars.insert(key.to_string(), value_parsed.to_string());
                        }
                    }
                }
            }
        }

        Ok(vars)
    }

    fn find(starting_dir: &Path, filename: &str) -> Option<PathBuf> {
        let mut dir = Some(starting_dir.to_path_buf());

        while let Some(current_dir) = dir {
            let candidate = current_dir.join(filename);

            if candidate.exists() {
                return Some(candidate);
            }

            // Remonter d'un répertoire
            dir = current_dir.parent().map(Path::to_path_buf);
        }

        None
    }

    pub fn get_var(&self, k: String) -> Option<String> {
        self.all_vars().get(&k.to_uppercase()).cloned()
    }

    pub fn has_var(&self, k: String) -> bool {
        self.all_vars().contains_key(&k.to_uppercase())
    }

    pub fn all_vars(&self) -> &HashMap<String, String> {
        &self.vars
    }

    pub fn set_var(&mut self, k: String, v: String) -> bool {

        let key_upper: String = k.to_uppercase();

        if self.has_var(key_upper.clone()) {
            false
        }
        else {
            self.vars.insert(key_upper, parse_var(v));
            true
        }
    }
}