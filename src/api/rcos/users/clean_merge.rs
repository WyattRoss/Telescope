use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use crate::api::rcos::users::clean_merge::clean_merge::ResponseData;


/// Type representing GraphQL mutation to merge two user accounts
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/clean_merge.graphql"
)]

pub struct CleanMerge;

impl CleanMerge{
    pub async fn execute(
        first_id: uuid,
        merge_id: uuid
        ) -> Result<ResponseData, TelescopeError> {
        send_query::<Self>(clean_merge::Variables {
            first_id,
            merge_id
        })
        .await
    }
}
