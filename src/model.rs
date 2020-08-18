use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Human {
    pub(crate) id: String,
    pub(crate) name: String,
}

#[juniper::object(description = "A human")]
impl Human {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
#[derive(juniper::GraphQLInputObject)]
pub struct NewHuman {
    pub(crate) id: String,
    pub(crate) name: String,
}

/**
 * Database State neccesary for rocket, wrap db connection inside of it or whatever you want.
 */
pub struct Database {
}


impl Database {

    pub fn new() -> Database {
        Database {
        }
    }
}
