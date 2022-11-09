use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing public RCOS projects.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/raw_projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]

pub struct RawProjects;

impl RawProjects{
    pub async fn get() -> Result<raw_projects::ResponseData, TelescopeError>{
        Ok(send_query::<Self>(raw_projects::Variables{})
            .await?)
    }
}
