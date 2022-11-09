use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use actix_web::web::Form;
use actix_web::web::{Path, ServiceConfig, HttpRequest};
use crate::api::rcos::prelude::*;
use serde::{
    de,
    Deserialize
};
use crate::api::rcos::users::enrollments::set_projecs_by_ids;
use crate::api::rcos::users::enrollments::set_projecs_by_ids::SetProjectsByIds;
use crate::api::rcos::projects::raw_projects::RawProjects;

const TEMPLATE_PATH: &str = "coordinate/set_projects";

pub fn register(config: &mut ServiceConfig){
    config
        .service(assignment_form);
}

#[get("/semesters/project_assignments/{semester_id}")]
pub async fn assignment_form(
    req: HttpRequest,
    Path(semester_id): Path<String>
) -> Result<Page, TelescopeError>{
    let semester_data = RawProjects::get().await?;
    
    let mut form = Template::new(TEMPLATE_PATH);
    form.fields = json!({
        "projects": semester_data.projects,
    });

    form.in_page(&req, "Assign Projects").await
}
