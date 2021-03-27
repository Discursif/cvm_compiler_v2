const ANY_TYPE: &'static str = "Bytes";
const VOID_TYPE: &'static str = "Empty";
const BYTE_TYPE: &'static str = "Byte";
const CHAR_TYPE: &'static str = "Char";
const PANIC_TYPE: &'static str = "Panic";
const STRING_TYPE: &'static str = "String";

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


#[derive(serde::Deserialize,serde::Serialize)]
pub struct TypeConfig {
    #[serde(default = "default_any_type")]
    any: String,
    #[serde(default = "default_void_type")]
    void: String,
    #[serde(default = "default_byte_type")]
    byte: String,
    #[serde(default = "default_char_type")]
    char: String,
    #[serde(default = "default_panic_type")]
    panic: String,
    #[serde(default = "default_string_type")]
    string: String,
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

#[derive(serde::Deserialize,serde::Serialize)]
pub struct OptimizerConfig {
    #[serde(default = "opt_on")]
    elide_unused_writes: bool,
    #[serde(default = "opt_on")]
    remove_followed_usages: bool,
    #[serde(default = "opt_on")]
    regroup_consts: bool,
    #[serde(default = "opt_on")]
    compile_time_evaluation: bool,
    #[serde(default = "opt_on")]
    function_inliner: bool,
    #[serde(default = "opt_on")]
    if_optimizer: bool,
    #[serde(default = "opt_on")]
    loop_break_inline: bool,
    #[serde(default = "opt_on")]
    remap_consts: bool,
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
            loop_break_inline: true,
            remap_consts: true,
        }
    }
}
#[derive(serde::Deserialize,serde::Serialize)]
pub enum OutputFormat {
    Binary,
    Asm,
    Lir,
    Mir,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Binary
    }
}

fn default_output_format() -> Vec<OutputFormat> {
    vec![OutputFormat::default()]
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct ProjectConfig {
    #[serde(default = "TypeConfig::default")]
    types: TypeConfig,
    #[serde(default = "OptimizerConfig::default")]
    optimizer: OptimizerConfig,
    #[serde(default = "default_output_format")]
    output_format: Vec<OutputFormat>,
}