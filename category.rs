use crate::project::Project;

pub fn count(

    list:&[Project],

    category:&str

)->usize{

    list.iter()

        .filter(|p|

            p.category==category

        )

        .count()

}
