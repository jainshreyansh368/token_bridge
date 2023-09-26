use rocket::{fairing::AdHoc, routes};

pub mod transaction;

pub fn mount() -> AdHoc {
    AdHoc::on_ignite("Attaching Routes", |rocket| async {
        rocket.mount("/", routes![])
    })
}
