use async_trait::async_trait;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, process::Command};

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
pub struct JsConfigOptions {
    /// Skips installing dependencies
    #[clap(short, long)]
    skip_dependencies: bool,
}

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

        let package_manager = if file_utils::does_file_exist("./yarn.lock") {
            "yarn"
        } else if file_utils::does_file_exist("./pnpm-lock.yaml") {
            "pnpm"
        } else {
            "npm"
        };

        if !self.skip_dependencies {
            println!("Install dependencies using {}", package_manager);

            let mut dependencies = vec![
                "@swc/core@latest",
                "@swc/jest@latest",
                "@swc/cli@latest",
                "prettier@latest",
                "prettier-config-get-off-my-lawn@latest",
                "eslint@latest",
                "eslint-config-get-off-my-lawn@latest",
                "jest@latest",
                "chance@latest",
                "nodemon@latest",
                "prettier-plugin-packagejson@latest",
            ];

            if is_typescript {
                dependencies.push("typescript@latest");
                dependencies.push("@types/node16@latest");
                dependencies.push("@types/jest@latest");
                dependencies.push("@types/chance@latest");
            }

            Command::new(package_manager)
                .arg("install")
                .arg("-D")
                .args(dependencies)
                .output()
                .unwrap();
        }

        println!("Creating a SWC config");

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

        println!("Creating a ESLint config");

        let eslint_config = r#"module.exports = {
    extends: ['get-off-my-lawn'],
};
"#;

        let js_file_ext = if is_esm { "cjs" } else { "js" };

        file_utils::create_and_write_file(
            format!("./.eslintrc.{}", js_file_ext).as_str(),
            eslint_config,
        )
        .unwrap();

        println!("Creating a Prettier config");

        let eslint_config = r#"module.exports = {
    ...require('prettier-config-get-off-my-lawn'),
    plugins: [require('prettier-plugin-packagejson')],
};
"#;

        let js_file_ext = if is_esm { "cjs" } else { "js" };

        file_utils::create_and_write_file(
            format!("./.prettierrc.{}", js_file_ext).as_str(),
            eslint_config,
        )
        .unwrap();

        anyhow::Ok(())
    }
}

#[derive(Subcommand, Debug)]
pub enum JsSubcommand {
    /// Creates the recommended swc config and tsconfig file based on the package.json
    Config(JsConfigOptions),
}
