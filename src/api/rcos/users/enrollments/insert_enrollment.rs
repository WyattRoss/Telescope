use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing GraphQL enrollment edit mutation.
#[derive(GraphQLQuery )]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/enrollments/insert_enrollment.graphql"
)]
pub struct InsertEnrollment;

impl InsertEnrollment {
    pub async fn execute(
        user_id: uuid,
        semester_id: String,
        credits: i64
        ) -> Result<uuid, TelescopeError> {
        send_query::<Self>(insert_enrollment::Variables{
            user_id,
            semester_id,
            credits,
        })
        .await
        .map(|data| data.insert_enrollments_one.unwrap().user_id)
    }
}
