//! Mutation to set multiple users to a single project

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing GraphQL project set mutation.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/enrollments/set_projects_by_ids.graphql",
    response_derives = "Debug,Copy,Clone,Serialize"
)]
pub struct SetProjectsByIds;

impl SetProjectsByIds{
    pub async fn execute(vars: set_projects_by_ids::Variables) -> Result<set_projects_by_ids::ResponseData, TelescopeError>{
       send_query::<Self>(vars)
           .await
    }
}
