use askama::Template;
use clap::{Arg, Command};
use colored::*;
use std::fs;
use std::io::{self};
use std::path::Path;

#[derive(Template)]
#[template(path = "README.md")]
struct Readme {
    name: String,
    path: String,
}

#[derive(Template)]
#[template(path = ".gitignore")]
struct Gitignore {
    content: String,
}

#[derive(Template)]
#[template(path = "CHANGELOG.md")]
struct Changelog {
    content: String,
}

#[derive(Template)]
#[template(path = "config.toml", escape = "txt")]
struct Config {
    name: String,
    path: String,
    version: String,
    description: String,
}

pub fn start() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            if let Some(name) = sub_matches.get_one::<String>("name") {
                println!("{} Initializing", "✓".green());

                // TODO: GENERATE empty folder
                // TODO: GENERATE toml file

                let path = format!("./examples/{}", name);
                match fs::create_dir(&path) {
                    Ok(_) => {
                        println!("{} Checking recipes", "✓".green());
                        generate_readme(name, &path).unwrap();
                        generate_gitignore(&path, ".DS_Store").unwrap();
                        generate_changelog(&path, "").unwrap();
                        generate_config(&path, name).unwrap();

                        println!("{} Generating recipe:", "✓".green());
                        println!("  {}", path);
                        let _ = generate_recipe_folder(path, name);
                        println!();
                        // println!("Run \"{} {}\"", "xef run".purple(), name.purple());
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        _ => unreachable!(),
    }
}

fn cli() -> Command {
    Command::new("xef")
        .about("Xef allows users to create and use reuse file/folders templates")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init").about("Generate a new recipe").arg(
                Arg::new("name")
                    .short('n')
                    .required(true)
                    .help("Name of the recipe"),
            ),
        )
        .subcommand(
            Command::new("run").about("Run an existing recipe").arg(
                Arg::new("name")
                    .short('n')
                    .required(true)
                    .help("Name of the recipe"),
            ),
        )
}

fn generate_readme(project_name: &str, path: &str) -> std::io::Result<()> {
    let readme = Readme {
        name: project_name.to_owned(),
        path: path.to_owned(),
    };

    let readme_path = format!("{}/README.md", path);

    fs::write(readme_path, readme.render().unwrap())
}

fn generate_gitignore(path: &str, content: &str) -> std::io::Result<()> {
    let tmpl = Gitignore {
        content: content.to_owned(),
    };

    let file_path = format!("{}/.gitignore", path);

    fs::write(file_path, tmpl.render().unwrap())
}

fn generate_changelog(path: &str, content: &str) -> std::io::Result<()> {
    let tmpl = Changelog {
        content: content.to_owned(),
    };

    let file_path = format!("{}/CHANGELOG.md", path);

    fs::write(file_path, tmpl.render().unwrap())
}

fn generate_config(path: &str, name: &str) -> std::io::Result<()> {
    let tmpl = Config {
        name: name.to_owned(),
        path: path.to_owned(),
        version: "0.1.0".to_owned(),
        description: "Example project".to_owned(),
    };

    let file_path = format!("{}/config.toml", path);

    fs::write(file_path, tmpl.render().unwrap())
}

fn generate_recipe_folder(path: String, name: &str) -> Result<String, io::Error> {
    let x = format!("{}/{}", path, "recipe");
    let recipe_path = Path::new(x.as_str());
    if recipe_path.exists() {
        return todo!();
    }

    match fs::create_dir(recipe_path) {
        Ok(_) => match fs::write(format!("{x}/hello.md"), "Hello, {{ name }}!") {
            Ok(_) => println!("Run \"{} {}\"", "xef run".purple(), name.purple()),
            Err(e) => {
                println!("{:?}", e);
                ()
            }
        },
        Err(e) => {
            println!("{:?}", e);
            ()
        }
    }

    Ok("dasd".to_owned())
}
