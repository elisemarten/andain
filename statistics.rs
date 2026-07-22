use crate::category;
use crate::project::Project;

pub fn print(

    list:&[Project]

){

    println!();

    println!("Statistics\n");

    println!(

        "Total Projects: {}",

        list.len()

    );

    println!(

        "Messaging: {}",

        category::count(

            list,

            "Messaging"

        )

    );

    println!(

        "Social Networks: {}",

        category::count(

            list,

            "Social Network"

        )

    );

    println!(

        "Web Services: {}",

        category::count(

            list,

            "Web Service"

        )

    );

}
