use std::path::Path;

use crate::CompilationContext;

use generator::doc_for_type;

use crate::Type;

pub mod generator;

pub fn export_doc_to(path: &Path, compiler_context: &CompilationContext) {
    let mut types: Vec<String> = compiler_context.types.keys().cloned().collect();
    types.insert(0, "global".to_owned());
    std::fs::write(path.join("index.js"), include_bytes!("index.js")).unwrap();
    std::fs::write(path.join("style.css"), include_bytes!("style.css")).unwrap();
    std::fs::write(
        path.join("right-arrow.svg"),
        include_bytes!("right-arrow.svg"),
    )
    .unwrap();
    let mut ty: Type = Type::new(
        "global".to_owned(),
        vec![
            "This object is the global scope object".to_owned(),
            "It mainly contains all global functions".to_owned(),
        ],
    );
    ty.static_functions = compiler_context.functions.clone();
    std::fs::write(
        path.join(format!("type_global.html")),
        doc_for_type(&ty, &types),
    )
    .unwrap();
    compiler_context.types.iter().for_each(|(_, x)| {
        std::fs::write(
            path.join(format!("type_{}.html", x.name)),
            doc_for_type(x, &types),
        )
        .unwrap();
    })
}
