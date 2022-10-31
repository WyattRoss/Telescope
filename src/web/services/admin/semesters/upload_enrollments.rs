use csv::ReaderBuilder;
use serde::Deserialize;
use std::io::prelude::*;
use crate::api::rcos::users::enrollments::insert_enrollment::InsertEnrollment;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use actix_easy_multipart::{MultipartFile, FromMultipart};
use actix_easy_multipart::extractor::MultipartForm;
use actix_web::web::HttpRequest;
use actix_web::web::{self as aweb, Path, ServiceConfig};
use crate::api::rcos::users::UserAccountType;
use crate::api::rcos::users::UserRole;
use crate::api::rcos::users::create::CreateOneUser;
use crate::api::rcos::users::accounts::user_accounts_by_rcsid::UserAccountsByRCSID;

const TEMPLATE_PATH: &str = "admin/semesters/forms/upload";

pub fn register(conf: &mut ServiceConfig) {
    conf
    .route(
        "/semesters/{semester_id}/upload",
        aweb::get().to(upload_page)
        )
    .route(
        "/semesters/{semester_id}/upload",
        aweb::post().to(submit_upload)
    );
}

pub async fn upload_page(
req: HttpRequest,
Path(_semester_id): Path<String>,
) -> Result<Page, TelescopeError>{
    let form = Template::new(TEMPLATE_PATH);
    form.in_page(&req, "Upload Enrollments").await
}

pub async fn submit_upload(
req: HttpRequest,
Path(semester_id): Path<String>,
MultipartForm(form): MultipartForm<UploadForm>
) -> Result<Page, TelescopeError>{
    let mut csv_file = form.csv_upload.file.as_file();
    let mut content = String::new();
    csv_file.read_to_string(&mut content).unwrap();

    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_reader(content.as_bytes());

    for result in reader.deserialize() {

        let enrollment: EnrollmentUpload = match result{
            Ok(roll) => roll,
            Err(_e) => return Err(TelescopeError::CSVError),
        };

        let account_get = UserAccountsByRCSID::get(enrollment.rcs_id.clone()).await?; 

        let num_accounts = match account_get.user_accounts_aggregate.aggregate{
            Some(val) => val.count,
            None => 0,
        };

        let user_id = match num_accounts{
            0 => CreateOneUser::execute(enrollment.first_name, enrollment.last_name, UserRole::Student, UserAccountType::Rpi, enrollment.rcs_id).await?.unwrap(),
            _ => account_get.user_accounts.get(0).unwrap().user_id, //should be safe

        };
        InsertEnrollment::execute(user_id, semester_id.clone(), form.credits).await?;

    }
 
    let form = Template::new(TEMPLATE_PATH);
    form.in_page(&req, "Upload Enrollments").await
}

#[derive(FromMultipart, Debug)]
pub struct UploadForm {
    csv_upload: MultipartFile,
    credits: i64,
}

#[derive(Deserialize, Debug)]
pub struct EnrollmentUpload {
    _timestamp: String,
    first_name: String,
    last_name: String,
    rcs_id: String,
    _discord_id: String,
    _project: String,
    _graduation_year: i32,

}
