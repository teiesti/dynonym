use model::User;

#[get("/dns/update")]
fn update(user: User) -> String {
    format!("{:?}", user)
}
