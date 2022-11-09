//! RCOS API query to get user IDs from a list of account ids for a given platform

use crate::api::rcos::prelude::*;
use crate::api::rcos::users::UserAccountType as user_account;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/user_accounts_lookup.graphql"
)]
pub struct UserAccountsLookup;

impl UserAccountsLookup {
    pub async fn execute(
        platform: user_account,
        platform_ids: Vec<String>
        ) -> Result<user_accounts_lookup::ResponseData, TelescopeError>{
        send_query::<Self>(user_accounts_lookup::Variables{
            account_ids: Some(platform_ids),
            account_type: platform,
        }).await
    }
}
