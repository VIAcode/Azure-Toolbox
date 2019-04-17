extern crate clap;
extern crate templatevalidator;

use clap::{App, Arg};
use templatevalidator::{Files, Templates};

fn main() {
    let args = App::new("ARMTemplateValidator")
        .version("0.1.0")
        .author("Sergei Kiselev <fffgghh@list.ru>")
        .about("Azure ARM template checker. Valid JSON + Valid data in params and values")
        .arg(
            Arg::with_name("path")
                .required(true)
                .index(1)
                .multiple(true)
                .help(
                    "Path to template files. Valid values: \".\", \"*\", \".*\", \"*.json\", \
                     \"./azuredeploy.json\", \"azuredeploy.json folder/azure_deploy2.json\"",
                ),
        )
        .get_matches();

    let input_path = args.values_of_lossy("path").unwrap();
    let all_files = Files::new(input_path);
    let templates = Templates::new(&all_files);
    templates.validate();
}
