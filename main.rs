mod category;
mod config;
mod project;
mod renderer;
mod repository;
mod sample_data;
mod search;
mod statistics;
mod timeline;

use repository::Repository;
use search::Search;

fn main() {

    let projects =

        Repository::load();

    println!(

        "Searching: {}\n",

        config::SEARCH_TERM

    );

    if let Some(item)=

        Search::by_name(

            &projects,

            config::SEARCH_TERM

        ){

        renderer::show(

            &item

        );

    }

    println!();

    timeline::build(

        &projects

    );

    println!();

    statistics::print(

        &projects

    );

}
