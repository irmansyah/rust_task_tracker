use actix_web::{web, Scope};

use crate::adapters::api::users::users_controllers::{delete_one_user_by_id, get_all_user, get_one_user_by_id, login_user, register_user, update_one_user};

pub fn routes() -> Scope {
    web::scope("/messages")
        .service(register_user)
        .service(login_user)
        .service(update_one_user)
        .service(get_all_user)
        .service(get_one_user_by_id)
        .service(delete_one_user_by_id)
}
