use crate::{
    function::Function,
    types::{Type, Variant},
};

pub fn doc_for_type(ty: &Type, all_types: &Vec<String>) -> String {
    let methods = if ty.functions.is_empty() {
        String::new()
    } else {
        let mut panics = ty
            .functions
            .values()
            .map(|x| {
                x.functions
                    .iter()
                    .filter(|x| x.return_type == crate::PANIC_TYPE)
                    .map(generate_func_doc)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
            .join("");
        let mut normals = ty
            .functions
            .values()
            .map(|x| {
                x.functions
                    .iter()
                    .filter(|x| x.return_type != crate::PANIC_TYPE)
                    .map(generate_func_doc)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
            .join("");
        format!(
            r#"<div class="colapsable" id="method-container">
                <h2><img src="right-arrow.svg" height="16"> Methods <a class="close-docs"></a></h2>
                {normals}
                {panics}
            </div>"#,
        )
    };

    let static_methods = if ty.static_functions.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="colapsable" id="static-method-container">
                <h2><img src="right-arrow.svg" height="16"> Static methods <a class="close-docs"></a></h2>
                {}
            </div>"#,
            ty.static_functions
                .values()
                .map(|x| x
                    .functions
                    .iter()
                    .map(generate_func_doc)
                    .collect::<Vec<_>>())
                .flatten()
                .collect::<Vec<_>>()
                .join("")
        )
    };

    let references = if ty.variants.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="colapsable" id="ref-container">
            <h2><img src="right-arrow.svg" height="16"> References <a class="close-docs"></a></h2>
            {}
            </div>"#,
            ty.variants
                .values()
                .map(generate_ref_doc)
                .collect::<Vec<_>>()
                .join("")
        )
    };

    let docu = if ty.comment.is_empty() {
        String::new()
    } else {
        format!(
            r#"
        <div class="colapsable">
            <h2><img src="right-arrow.svg" height="16"> Documentation</h2>
            <div class="doc">
            {}
            </div>
        </div>"#,
            html_from_comments(&ty.comment)
        )
    };

    let mut meta = format!(
        r#"<div>parent <a class="link link-type" href="{}">{}</a></div>"#,
        build_url_for_type(&ty.parent),
        &ty.parent
    );

    if let Some(e) = ty.size {
        meta.push_str(&format!("size {}", e));
    }

    include_str!("type_template.html")
        .replace(
            "<!--SIDEBAR TYPES-->",
            &all_types
                .iter()
                .map(|x| {
                    format!(
                        "<li><a class=\"link link-type\" href=\"{}\">{x}</a></li>",
                        build_url_for_type(x)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .replace("<!--METHODS-->", &methods)
        .replace("<!--STATIC METHODS-->", &static_methods)
        .replace("<!--VARIANTS-->", &references)
        .replace("<!--TYPE DOC-->", &docu)
        .replace("<!--TYPE DOC-->", &docu)
        .replace("<!--META-->", &meta)
        .replace("<!--TYPE NAME-->", &ty.name)
        .replace("<!--TYPE LINK-->", &build_url_for_type(&ty.name))
}

fn generate_func_doc(func: &Function) -> String {
    let args = func
        .arguments
        .iter()
        .map(|x| {
            format!(
                r#"<a class="link link-type" href="{}">{}</a> <a class="link link-var">{}</a>"#,
                build_url_for_type(&x.var_type),
                x.var_type,
                x.name
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let (colap, img, doc) = if !is_doc_empty(&func.comments) {
        if func.return_type == crate::PANIC_TYPE {
            (
                " colapsable",
                r#"<img src="right-arrow.svg" height="16"> "#,
                format!(
                    "<div class=\"doc\"><span style=\"color:#f1c40f\">⚠ This function if called will crash the compiler</span><br><br>{}</div>",
                    html_from_comments(&func.comments)
                ),
            )
        } else {
            (
                " colapsable",
                r#"<img src="right-arrow.svg" height="16"> "#,
                format!(
                    "<div class=\"doc\">{}</div>",
                    html_from_comments(&func.comments)
                ),
            )
        }
    } else {
        if func.return_type == crate::PANIC_TYPE {
            (
                " colapsable",
                r#"<img src="right-arrow.svg" height="16"> "#,
                "<div class=\"doc\"><span style=\"color:#f1c40f\">⚠ This function if called will crash the compiler</span></div>"
                    .to_owned(),
            )
        } else {
            ("", "", String::new())
        }
    };
    let return_type = if func.return_type.trim().is_empty() {
        "Empty"
    } else {
        &func.return_type
    };
    let name = &func.name;
    if func.return_type == crate::PANIC_TYPE {
        format!(
            r#"<div class="method{colap}"><div class="definition">{img}<a class="link link-fn"><del>{name}</del> </a>({args}) <span class="arrow">→</span> <a class="link link-type">{return_type}</a></div>{doc}</div>"#,
        )
    } else {
        format!(
            r#"<div class="method{colap}"><div class="definition">{img}<a class="link link-fn">{name} </a>({args}) <span class="arrow">→</span> <a class="link link-type" href="{}">{return_type}</a></div>{doc}</div>"#,
            build_url_for_type(return_type)
        )
    }
}

fn is_doc_empty(doc: &[String]) -> bool {
    if doc.is_empty() {
        return true;
    }
    !doc.iter().any(|x| !x.trim().is_empty())
}

fn generate_ref_doc(refs: &Variant) -> String {
    let (colap, img, doc) = if !is_doc_empty(&refs.comment) {
        (
            " colapsable",
            r#"<img src="right-arrow.svg" height="16"> "#,
            format!(
                "<div class=\"doc\">{}</div>",
                html_from_comments(&refs.comment)
            ),
        )
    } else {
        ("", "", String::new())
    };
    let name = &refs.name;
    format!(
        r#"<div class="ref-element{colap}">
        <div class="definition">
            {img}<a class="link link-ref">{name}</a>
        </div>
        {doc}
    </div>"#
    )
}

fn html_from_comments(comments: &Vec<String>) -> String {
    comments
        .iter()
        .map(|x| {
            let x = x.trim();
            let mut x = if x.starts_with("///") {
                x[3..].trim().to_owned()
            } else {
                x.to_owned()
            };
            while let Some(e) = x.find("[type:") {
                let after = &x[e..];
                let end = after.find("]").unwrap();
                let inner = &after[6..end];

                x = format!(
                    r#"{}<a class="link link-type" href="{}">{}</a>{}"#,
                    &x[..e],
                    build_url_for_type(inner),
                    inner,
                    &after[end + 1..]
                );
            }
            while let Some(e) = x.find("[var:") {
                let after = &x[e..];
                let end = after.find("]").unwrap();
                let inner = &after[5..end];

                x = format!(
                    r#"{}<a class="link link-var">{}</a>{}"#,
                    &x[..e],
                    inner,
                    &after[end + 1..]
                );
            }
            while let Some(e) = x.find("[ref:") {
                let after = &x[e..];
                let end = after.find("]").unwrap();
                let inner = &after[5..end];

                x = format!(
                    r#"{}<a class="link link-ref">{}</a>{}"#,
                    &x[..e],
                    inner,
                    &after[end + 1..]
                );
            }
            x
        })
        .collect::<Vec<_>>()
        .join("<br>")
}

fn build_url_for_type(t: &str) -> String {
    format!("./type_{}.html", t)
}
