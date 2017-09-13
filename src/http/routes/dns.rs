use model::Credentials;

#[get("/dns/update")]
fn update(creds: Credentials) -> String {
    format!("{:?}", creds)
}
