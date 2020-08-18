use crate::{collection, model::{Human, Database, NewHuman}};
use juniper::{FieldError, FieldResult};
use bson::doc;

pub fn create_human(new_human: NewHuman) -> Result<Human, FieldError>{
    let coll = collection("human");
    let serialized_member = bson::to_bson(&new_human)?; 

    if let bson::Bson::Document(document) = serialized_member {
        coll.insert_one(document, None)?;
        let member_document = coll.find_one(Some(doc! { "id" => new_human.id }), None)?
                .expect("Document not found");

        let human = bson::from_bson(bson::Bson::Document(member_document))?;
        Ok(human)
        
        
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }
}

pub fn get_human(id: &str) -> Result<Vec<Human>,FieldError> {
    let coll = collection("human");
    let filter = doc! {"id" : id};
    let cursor = coll.find(filter, None).unwrap();

    let mut results: Vec<Human> = vec![];

    for result in cursor {
        match result {
            Ok(doc) => {
            let human: Option<Human> = bson::from_bson(bson::Bson::Document(doc)).ok();
            println!("res: {:?}", human);
            results.push(human.unwrap());
            },
            Err(error) => {
                println!("Error to find doc: {}", error);
            } 
        }
    }
    Ok(results)
}
    

pub struct Query;

#[juniper::object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
)]
/// The root query object of the schema
impl Query {
    #[graphql(arguments(id(description = "id of the human")))]
    fn human(database: &Database, id: String) -> FieldResult<Vec<Human>> {
        get_human(&id)
    }
}

pub struct MutationRoot;

#[juniper::object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
)]
impl MutationRoot {
    fn create_human(database: &Database, new_human: NewHuman) -> FieldResult<Human> {
        create_human(new_human)
    }
}
