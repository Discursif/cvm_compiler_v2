use std::path::Path;

fn default_any_type() -> String {
    return "Bytes".to_owned();
}

fn default_void_type() -> String {
    return "Empty".to_owned();
}

fn default_byte_type() -> String {
    return "Byte".to_owned();
}

fn default_char_type() -> String {
    return "Char".to_owned();
}

fn default_panic_type() -> String {
    return "Panic".to_owned();
}

fn default_string_type() -> String {
    return "String".to_owned();
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TypeConfig {
    #[serde(default = "default_any_type")]
    pub any: String,
    #[serde(default = "default_void_type")]
    pub void: String,
    #[serde(default = "default_byte_type")]
    pub byte: String,
    #[serde(default = "default_char_type")]
    pub char: String,
    #[serde(default = "default_panic_type")]
    pub panic: String,
    #[serde(default = "default_string_type")]
    pub string: String,
}

impl Default for TypeConfig {
    fn default() -> Self {
        Self {
            any: default_any_type(),
            void: default_void_type(),
            byte: default_byte_type(),
            char: default_char_type(),
            panic: default_panic_type(),
            string: default_string_type(),
        }
    }
}

fn opt_on() -> bool {
    return true;
}

fn opt_off() -> bool {
    return false;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OptimizerConfig {
    #[serde(default = "opt_on")]
    pub elide_unused_writes: bool,
    #[serde(default = "opt_on")]
    pub clear_unreachable: bool,
    #[serde(default = "opt_on")]
    pub loop_fn_return: bool,
    #[serde(default = "opt_on")]
    pub remove_followed_usages: bool,
    #[serde(default = "opt_on")]
    pub regroup_consts: bool,
    #[serde(default = "opt_on")]
    pub compile_time_evaluation: bool,
    #[serde(default = "opt_on")]
    pub function_inliner: bool,
    #[serde(default = "opt_on")]
    pub if_optimizer: bool,
    #[serde(default = "opt_off")]
    pub loop_break_inline: bool,
    #[serde(default = "opt_on")]
    pub remap_consts: bool,
    #[serde(default = "opt_off")]
    pub infer_sizes_from_meta: bool,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            elide_unused_writes: true,
            remove_followed_usages: true,
            regroup_consts: true,
            compile_time_evaluation: true,
            function_inliner: true,
            if_optimizer: true,
            loop_break_inline: false,
            remap_consts: true,
            clear_unreachable: true,
            loop_fn_return: true,
            infer_sizes_from_meta: false,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum OutputFormat {
    Binary,
    Asm,
    Lir,
    Mir,
    Python,
    C,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Binary
    }
}

fn default_output_format() -> Vec<OutputFormat> {
    vec![OutputFormat::default()]
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ProjectConfig {
    #[serde(default = "TypeConfig::default")]
    pub types: TypeConfig,
    #[serde(default = "OptimizerConfig::default")]
    pub optimizer: OptimizerConfig,
    #[serde(default = "CompilerConfig::default")]
    pub compiler: CompilerConfig,
    #[serde(default = "default_output_format")]
    pub output_format: Vec<OutputFormat>,
    #[serde(default = "default_output_folder")]
    pub output_folder: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CompilerConfig {
    #[serde(default = "opt_on")]
    pub emit_meta_type_size: bool,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            emit_meta_type_size: true,
        }
    }
}

fn default_output_folder() -> String {
    "build".to_owned()
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            types: TypeConfig::default(),
            optimizer: OptimizerConfig::default(),
            output_format: vec![OutputFormat::Binary],
            output_folder: "build".to_owned(),
            compiler: CompilerConfig::default(),
        }
    }
}

pub fn load_config(project_dir: &str) -> ProjectConfig {
    let file = format!("{}/config.yml", project_dir).replace("//", "/");
    if !Path::new(&file).exists() {
        println!("`{}` doesn't exists! Using default config instead!", file);
        let default_config = ProjectConfig::default();
        match serde_yaml::to_string(&default_config).map(|x| std::fs::write(&file, x)) {
            Ok(Ok(_)) => {
                println!("A default `config.yml` has been created in your project folder")
            }
            Ok(Err(e)) => {
                println!("Can't write in `{}` file: {}", file, e);
            }
            Err(e) => {
                println!("Can't serialize to YAML default config: {}", e);
            }
        }

        return default_config;
    }
    match std::fs::read_to_string(&file) {
        Ok(eio) => match serde_yaml::from_str(&eio) {
            Ok(e) => match serde_yaml::to_string(&e) {
                Ok(ep) => {
                    if ep != eio {
                        match std::fs::write(&file, ep) {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Can't write in `{}` file: {}", file, e);
                            }
                        }
                    }
                    return e;
                }
                Err(er) => {
                    println!("Can't serialize to YAML the config: {}", er);
                    return e;
                }
            },
            Err(e) => {
                println!("Can't deserialize to YAML the config: {}", e);
                ProjectConfig::default()
            }
        },
        Err(e) => {
            println!(
                "Can't read file `{}` using default config instead: {}",
                file, e
            );
            ProjectConfig::default()
        }
    }
}
