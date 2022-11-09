use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use actix_web::web::Form;
use actix_web::web::{Path, ServiceConfig, HttpRequest};
use crate::api::rcos::prelude::*;
use serde::Deserialize;
use crate::api::rcos::users::enrollments::set_projecs_by_ids;
use crate::api::rcos::users::enrollments::set_projecs_by_ids::SetProjectsByIds;
use crate::api::rcos::projects::raw_projects::RawProjects;
use crate::api::rcos::users::UserAccountType as user_account;
use crate::api::rcos::users::accounts::user_accounts_lookup::UserAccountsLookup;

const TEMPLATE_PATH: &str = "coordinate/set_projects";

pub fn register(config: &mut ServiceConfig){
    config
        .service(assignment_form)
        .service(submit_assignments);
}

#[get("/semesters/project_assignments/{semester_id}")]
pub async fn assignment_form(
    req: HttpRequest,
    Path(_semester_id): Path<String>
) -> Result<Page, TelescopeError>{
    let semester_data = RawProjects::get().await?;
    
    let mut form = Template::new(TEMPLATE_PATH);
    form.fields = json!({
        "projects": semester_data.projects,
    });

    form.in_page(&req, "Assign Projects").await
}

#[post("/semesters/project_assignments/{semester_id}")]
pub async fn submit_assignments(
    req: HttpRequest,
    Form(form_data): Form<FormData>,
    Path(semester_id): Path<String>
) -> Result<Page, TelescopeError>{

    let FormData {
        project_id,
        rcs_ids,
    } = form_data;
    
    dbg!(rcs_ids.clone());
    let mut ids_string = match rcs_ids{
        Some(s) => s,
        None => String::new(),
    };
    dbg!(ids_string.clone());

    ids_string.retain(|c| !c.is_whitespace());
    dbg!(ids_string.clone());

    let id_vec: Vec<String> = {
        let e: Vec<&str> = ids_string.split(',').collect();
        e.iter().map(|s| s.to_string()).collect()
    };

    let uuids: Vec<uuid>= UserAccountsLookup::execute(user_account::Rpi, id_vec).await?.user_accounts.iter()
        .map(|val| val.user_id).collect();
    dbg!(uuids.clone());

    SetProjectsByIds::execute(set_projecs_by_ids::set_projects_by_ids::Variables { users: Some(uuids), semester_id, project_id }).await?;

    let semester_data = RawProjects::get().await?;
    
    let mut form = Template::new(TEMPLATE_PATH);
    form.fields = json!({
        "projects": semester_data.projects,
    });

    form.in_page(&req, "Assign Projects").await
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormData  {
    project_id: Option<i64>,
    rcs_ids: Option<String>,
}
