use crate::project::Project;

pub struct Search;

impl Search{

    pub fn by_name(

        list:&[Project],

        text:&str

    )->Option<Project>{

        list.iter()

            .find(|p|

                p.name.eq_ignore_ascii_case(text)

            )

            .cloned()

    }

}
