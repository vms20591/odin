use std::error::Error;

mod cli;
mod log;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli::parse_args();
    let file = matches.value_of("file");
    let application = app::App::new(file);

    if let Some(matches) = matches.subcommand_matches("list") {
        if matches.is_present("models") {
            if matches.is_present("all") {
                application.list_models_for_all()?;
            }
            else if let Some(brand) = matches.value_of("brand") {
                application.list_models_for(brand)?;
            }
            else {
                println!("{}", matches.usage());
            }
        }
        else {
            application.list_brands()?;
        }
    }
    else if let Some(_) = matches.subcommand_matches("init") {
        application.init();
    }
    else {
        println!("{}", matches.usage());
    }

    Ok(())
}
