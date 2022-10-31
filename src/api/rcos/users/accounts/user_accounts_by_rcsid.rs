use crate::error::TelescopeError;
use crate::api::rcos::{prelude::*, send_query};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/user_accounts_by_rcsid.graphql",
    response_derives = "Debug,Clone,Serialize"
)]

pub struct UserAccountsByRCSID;

impl UserAccountsByRCSID {
    pub async fn get(
        account_id: String,
        ) -> Result<user_accounts_by_rcsid::ResponseData, TelescopeError> {
        send_query::<Self>(user_accounts_by_rcsid::Variables {
            account_id
        })
        .await
    }
}
