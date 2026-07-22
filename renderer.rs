use crate::project::Project;

pub fn show(

    project:&Project

){

    println!("Project\n");

    println!("{}",project.name);

    println!();

    println!("Category");

    println!("{}",project.category);

    println!();

    println!("Released");

    println!("{}",project.launched);

    println!();

    println!("Closed");

    println!("{}",project.closed);

    println!();

    println!("Description");

    println!("{}",project.description);

}
