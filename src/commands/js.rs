use async_trait::async_trait;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::utils::file_utils;

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
struct PackageJson {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    the_type: Option<String>,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
}

fn has_key(map: &Option<HashMap<String, String>>, key: &str) -> bool {
    map.is_some() && map.as_ref().unwrap().contains_key(key)
}

impl PackageJson {
    fn new() -> anyhow::Result<Self> {
        match fs::read_to_string("./package.json") {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(package_json) => Ok(package_json),
                Err(e) => anyhow::bail!("Failed to parse package.json: {}", e),
            },
            Err(e) => anyhow::bail!("Failed to read package.json: {}", e),
        }
    }

    fn has_dependency(&self, name: &str) -> bool {
        has_key(&self.dependencies, name) || has_key(&self.dev_dependencies, name)
    }

    fn is_esm(&self) -> bool {
        self.the_type.is_some() && self.the_type.as_ref().unwrap() == "module"
    }
}

#[derive(Parser, Debug)]
pub struct JsConfigOptions {}

#[async_trait]
impl super::CommandRunner for JsConfigOptions {
    async fn execute(&self) -> anyhow::Result<()> {
        let package_json = PackageJson::new().unwrap();

        let is_esm = package_json.is_esm();
        let is_typescript = package_json.has_dependency("typescript");

        println!("This repo appears to be using {}. If this is incorrect, please set \"type\": \"{}\" in package.json.", if is_esm { "ES Modules" } else { "Common JS" }, if is_esm { "commonjs" } else { "module" });
        println!(
            "This repo {} to be using TypeScript. If this is incorrect, please {} the \"typescript\" dependency in package.json.",
            if is_typescript {
                "appears"
            } else {
                "does not appear"
            },
            if is_typescript {
                "remove"
            } else {
                "add"
            }
        );

        println!("\nCreating a SWC config");

        let syntax = if is_typescript {
            "typescript"
        } else {
            "ecmascript"
        };

        let (target, the_type) = if is_esm {
            ("es2020", "es6")
        } else {
            ("es5", "commonjs")
        };

        let swc_config = format!(
            r#"{{
    "jsc": {{
        "parser": {{
            "syntax": "{}"
        }},
        "target": "{}"
    }},
    "minify": true,
    "module": {{
        "strict": true,
        "strictMode": true,
        "type": "{}"
    }},
    "sourceMaps": "inline"
}}"#,
            syntax, target, the_type
        );

        file_utils::create_and_write_file("./.swcrc", swc_config).unwrap();

        if is_typescript {
            println!("Creating a TSConfig");

            let tsconfig = format!(
                r#"{{
    "$schema": "https://raw.githubusercontent.com/SchemaStore/schemastore/master/src/schemas/json/tsconfig.json",
    "extends": "lsctl/tsconfig/{}-server.json"
}}"#,
                if is_esm { "esm" } else { "cjs" }
            );

            file_utils::create_and_write_file("./tsconfig.json", tsconfig).unwrap();
        }

        anyhow::Ok(())
    }
}

#[derive(Subcommand, Debug)]
pub enum JsSubcommand {
    /// Creates the recommended swc config and tsconfig file based on the package.json
    Config(JsConfigOptions),
}
