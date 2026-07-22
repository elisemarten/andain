use crate::project::Project;
use crate::sample_data;

pub struct Repository;

impl Repository{

    pub fn load()->Vec<Project>{

        sample_data::load()

    }

}
