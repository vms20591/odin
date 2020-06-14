use std::error::Error;

mod cli;
mod log;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli::parse_args();

    if let Some(matches) = matches.subcommand_matches("list") {
        if matches.is_present("models") {
            if matches.is_present("all") {
                app::list_models_for_all()?;
            }
            else if let Some(brand) = matches.value_of("brand") {
                app::list_models_for(brand)?;
            }
            else {
                println!("{}", matches.usage());
            }
        }
        else {
            app::list_brands()?;
        }
    }
    else {
        println!("{}", matches.usage());
    }

    Ok(())
}
