use crate::project::Project;

pub fn load() -> Vec<Project> {

    vec![

        Project{

            name:"ICQ".into(),

            category:"Messaging".into(),

            launched:1996,

            closed:2024,

            description:"One of the first popular instant messengers.".into()

        },

        Project{

            name:"MySpace".into(),

            category:"Social Network".into(),

            launched:2003,

            closed:2019,

            description:"Former social networking platform.".into()

        },

        Project{

            name:"Orkut".into(),

            category:"Social Network".into(),

            launched:2004,

            closed:2014,

            description:"Google social networking service.".into()

        },

        Project{

            name:"Google Friend Connect".into(),

            category:"Web Service".into(),

            launched:2008,

            closed:2012,

            description:"Widget platform for websites.".into()

        },

        Project{

            name:"Windows Live Messenger".into(),

            category:"Messaging".into(),

            launched:1999,

            closed:2014,

            description:"Microsoft messaging platform.".into()

        },

        Project{

            name:"Google+".into(),

            category:"Social Network".into(),

            launched:2011,

            closed:2019,

            description:"Google social network.".into()

        }

    ]

}
