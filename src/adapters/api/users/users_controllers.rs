use crate::{
    adapters::api::{
        shared::{app_state::AppState, error_presenter::ErrorResponse},
        users::{users_mappers::UserPresenterMapper, users_payloads::{UserPayload, UserRegisterPayload}, users_presenters::UserRegisterPresenter},
    },
    application::{
        mappers::api_mapper::{ApiMapper, BaseResponse},
        usecases::{
            delete_one_user_by_id_usecase::DeleteOneUserByIdUseCase, get_all_users_usecase::GetAlluserUseCase, get_one_user_by_id_usecase::GetOneUserByIdUseCase, interfaces::AbstractUseCase, login_user_usecase::LoginUserUseCase, register_user_usecase::RegisterUserUseCase, update_one_user_usecase::UpdateOneUserUseCase
        },
    },
    domain::{error::ApiError, users_entity::UserEntity},
};
use actix_web::{delete, get, patch, post, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user)
        .service(login_user)
        .service(update_one_user)
        .service(get_all_user)
        .service(get_one_user_by_id)
        .service(delete_one_user_by_id);
}

#[post("/register")]
async fn register_user(data: web::Data<AppState>, path: web::Json<UserRegisterPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let post_one_user_usecase = RegisterUserUseCase::new(&user_payload, &data.user_repository);

    match post_one_user_usecase.execute().await {
        Ok(user) => {
            let response = BaseResponse {
                code: 201,
                message: "User created successfully".to_string(),
                data: UserPresenterMapper::to_api(user),
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[post("/login")]
async fn login_user(data: web::Data<AppState>, path: web::Json<UserRegisterPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let post_one_user_usecase = LoginUserUseCase::new(&user_payload, &data.user_repository);

    match post_one_user_usecase.execute().await {
        Ok(user) => {
            let response = BaseResponse {
                code: 201,
                message: "User created successfully".to_string(),
                data: UserPresenterMapper::to_api(user),
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[patch("/")]
async fn update_one_user(data: web::Data<AppState>, path: web::Json<UserPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let update_one_user_usecase = UpdateOneUserUseCase::new(&user_payload, &data.user_repository);

    match update_one_user_usecase.execute().await {
        Ok(user) => {
            let response = BaseResponse {
                code: 200,
                message: "User updated successfully".to_string(),
                data: UserPresenterMapper::to_api(user),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/")]
async fn get_all_user(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_user_usecase = GetAlluserUseCase::new(&data.user_repository);
    let user: Result<Vec<UserEntity>, ApiError> = get_all_user_usecase.execute().await;

    match user {
        Ok(user) => {
            let response = BaseResponse {
                code: 200,
                message: "User list retrieved successfully".to_string(),
                data: user.into_iter().map(UserPresenterMapper::to_api).collect::<Vec<UserPresenter>>(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/{user_id}")]
async fn get_one_user_by_id(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner().0;
    let get_one_user_by_id_usecase = GetOneUserByIdUseCase::new(&user_id, &data.user_repository);

    match get_one_user_by_id_usecase.execute().await {
        Ok(user) => {
            let response = BaseResponse {
                code: 200,
                message: "User retrieved successfully".to_string(),
                data: UserPresenterMapper::to_api(user),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[delete("/{user_id}")]
async fn delete_one_user_by_id(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner().0;
    let delete_one_user_usecase = DeleteOneUserByIdUseCase::new(&user_id, &data.user_repository);

    match delete_one_user_usecase.execute().await {
        Ok(user) => {
            let response = BaseResponse {
                code: 200,
                message: "User deleted successfully".to_string(),
                data: UserPresenterMapper::to_api(user),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}
