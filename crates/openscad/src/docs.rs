use std::collections::HashMap;
use std::sync::LazyLock;

pub struct ModuleDocs {
    pub description: String,
    pub arguments: Vec<ModuleDocsArguments>,
    pub examples: Vec<String>,
}

pub struct ModuleDocsArguments {
    name: String,
    description: String,
    default: Option<String>,
}

impl Clone for ModuleDocs {
    fn clone(&self) -> Self {
        ModuleDocs {
            description: self.description.clone(),
            arguments: self.arguments.clone(),
            examples: self.examples.clone(),
        }
    }
}

impl Clone for ModuleDocsArguments {
    fn clone(&self) -> Self {
        ModuleDocsArguments {
            name: self.name.clone(),
            description: self.description.clone(),
            default: self.default.clone(),
        }
    }
}

impl ModuleDocs {
    pub fn to_markdown(&self) -> String {
        let mut result = format!("**Description:** {}", self.description);

        if !self.arguments.is_empty() {
            result += "\n\n### Arguments:";
            for argument in &self.arguments {
                result += &format!("\n- `{}` {}", argument.name, argument.description);
                if let Some(default) = &argument.default {
                    result += &format!("Default: {default}");
                }
            }
        }

        if !self.examples.is_empty() {
            result += "\n\n### Examples:\n```";
            for example in &self.examples {
                result += &format!("\n{}", example);
            }
            result += "\n```";
        }

        result
    }
}

static BUILTIN_MODULE_DOCS: LazyLock<HashMap<&'static str, ModuleDocs>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "translate",
        ModuleDocs {
            description: "Translates (moves) its child elements along the specified vector."
                .to_owned(),
            arguments: vec![ModuleDocsArguments {
                name: "v".to_owned(),
                description: "vector to translate shape along".to_owned(),
                default: None,
            }],
            examples: vec!["translate(v = [x, y, z]) { ... }".to_owned()],
        },
    );

    map.insert(
        "circle",
        ModuleDocs {
            description: "Creates a circle at the origin. All parameters, except r, must be named."
                .to_owned(),
            arguments: vec![
                ModuleDocsArguments {
                    name: "r".to_owned(),
                    description: "circle radius. r name is the only one optional with circle."
                        .to_owned(),
                    default: None,
                },
                ModuleDocsArguments {
                    name: "d".to_owned(),
                    description: "circle diameter.".to_owned(),
                    default: None,
                },
            ],
            examples: vec![
                "circle(10);".to_owned(),
                "circle(r=10);".to_owned(),
                "circle(d=20);".to_owned(),
            ],
        },
    );

    map
});

pub fn get_builtin_module_docs(module_id: &str) -> Option<ModuleDocs> {
    BUILTIN_MODULE_DOCS.get(module_id).cloned()
}
