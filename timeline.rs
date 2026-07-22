use crate::project::Project;

pub fn build(

    list:&[Project]

){

    let mut items=list.to_vec();

    items.sort_by_key(|p|p.launched);

    println!("Timeline\n");

    for item in items{

        println!(

            "{} {}",

            item.launched,

            item.name

        );

    }

}
