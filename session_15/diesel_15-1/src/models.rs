#[derive(Queryable)]
pub struct Site {
    pub id: i32,
    pub site_id: i32,
    pub infra_md_ad: String,
    pub site_poc: String,
    pub contact_person: String,
    pub sub_contact_person: String,
    pub contact_ml: String,
    pub pjt_name: String,
    pub teams_channel: bool,
}
