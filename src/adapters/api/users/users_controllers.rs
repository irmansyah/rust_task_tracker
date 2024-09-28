use std::str::FromStr;

use crate::{
    adapters::api::{
        shared::{app_state::AppState, error_presenter::ErrorResponse, success_presenter::SuccessResponse},
        users::{
            users_mappers::{UserAccessTokenPresenterMapper, UserAllPresenterMapper, UserPresenterMapper},
            users_payloads::{UserIdPayload, UserLoginPayload, UserRefreshTokenPayload, UserRegisterPayload, UserRolePayload, UserUpdatePayload},
            users_presenters::UserAllPresenter,
        },
    },
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{
            interfaces::AbstractUseCase,
            user::{
                delete_one_user_by_id_usecase::DeleteOneUserByIdUseCase, get_all_users_usecase::GetAllUsersUseCase, get_one_user_by_id_usecase::GetOneUserByIdUseCase,
                login_user_usecase::LoginUserUseCase, refresh_token_user_usecase::RefreshTokenUserUseCase, register_user_usecase::RegisterUserUseCase,
                update_one_user_usecase::UpdateOneUserUseCase,
            },
        },
        utils::access_control::{auth_usecase::AuthCheckUseCase, extractors::claims::Claims},
    },
    domain::{error::ApiError, user_entity::{UserAllEntity, UserEntity}},
};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use reqwest::StatusCode;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user)
        .service(login_user)
        .service(get_refresh_token)
        .service(update_one_user)
        .service(update_one_user_own)
        .service(update_one_user_role)
        .service(get_all_users)
        .service(get_one_user_by_id)
        .service(get_one_user_by_id_own)
        .service(delete_one_user_by_id)
        .service(delete_one_user_by_id_own);
}

#[post("/register")]
async fn register_user(data: web::Data<AppState>, path: web::Json<UserRegisterPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let register_user_usecase = RegisterUserUseCase::new(&user_payload, &data.users_repository);

    match register_user_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::CREATED, "User created successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[post("/login")]
async fn login_user(data: web::Data<AppState>, path: web::Json<UserLoginPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let login_user_usecase = LoginUserUseCase::new(&user_payload, &data.users_repository);

    match login_user_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::OK, "User signin successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[post("/refresh")]
async fn get_refresh_token(data: web::Data<AppState>, path: web::Json<UserRefreshTokenPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let login_user_usecase = RefreshTokenUserUseCase::new(&user_payload, &data.users_repository);

    match login_user_usecase.execute().await {
        Ok(token) => Ok(SuccessResponse::new(StatusCode::OK, "Token generate successfully", UserAccessTokenPresenterMapper::to_api(token)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[patch("/one")]
async fn update_one_user(data: web::Data<AppState>, claims: Claims, path: web::Json<UserUpdatePayload>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let user_payload = path.into_inner();
    let update_one_user_usecase = UpdateOneUserUseCase::new(&user_payload, &data.users_repository);

    match update_one_user_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::OK, "User updated successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[patch("/one_own")]
async fn update_one_user_own(data: web::Data<AppState>, claims: Claims, path: web::Json<UserUpdatePayload>) -> Result<HttpResponse, ErrorResponse> {
    let mut user_payload = path.into_inner();
    user_payload.user_id = Some(claims.sub.clone());
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let update_one_user_usecase = UpdateOneUserUseCase::new(&user_payload, &data.users_repository);

    match update_one_user_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::OK, "User updated successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[patch("/one_role")]
async fn update_one_user_role(data: web::Data<AppState>, claims: Claims, path: web::Json<UserUpdatePayload>) -> Result<HttpResponse, ErrorResponse> {
    let mut user_payload = path.into_inner();
    let data_role = UserRolePayload::from_str(&claims.role.clone().to_string().as_str());
    user_payload.role = Some(data_role.clone().unwrap_or_default());

    if user_payload.user_id == Some(claims.sub.clone()) {
        return Err(ErrorResponse::map_io_error_default("Can't self promote!!!".to_string()));
    }

    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let update_one_user_usecase = UpdateOneUserUseCase::new(&user_payload, &data.users_repository);

    match update_one_user_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::OK, "User updated successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/all")]
async fn get_all_users(data: web::Data<AppState>, claims: Claims) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let get_all_users_usecase = GetAllUsersUseCase::new(&data.users_repository);
    let users: Result<Vec<UserAllEntity>, ApiError> = get_all_users_usecase.execute().await;

    match users {
        Ok(datas) => Ok(SuccessResponse::new(
            StatusCode::OK,
            "Users retrieved successfully",
            datas.into_iter().map(UserAllPresenterMapper::to_api).collect::<Vec<UserAllPresenter>>(),
        )
        .to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/one")]
async fn get_one_user_by_id(data: web::Data<AppState>, claims: Claims, path: web::Json<UserIdPayload>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let user_payload = path.into_inner();
    let get_one_user_by_id_usecase = GetOneUserByIdUseCase::new(&user_payload, &data.users_repository);

    match get_one_user_by_id_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::OK, "User retrieved successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/one_own")]
async fn get_one_user_by_id_own(data: web::Data<AppState>, claims: Claims) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = UserIdPayload { user_id: claims.sub.clone() };
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let get_one_user_by_id_usecase = GetOneUserByIdUseCase::new(&user_payload, &data.users_repository);

    match get_one_user_by_id_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::OK, "User retrieved successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[delete("/one")]
async fn delete_one_user_by_id(data: web::Data<AppState>, claims: Claims, path: web::Json<UserIdPayload>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let user_payload = path.into_inner();
    let delete_one_user_usecase = DeleteOneUserByIdUseCase::new(&user_payload, &data.users_repository);

    match delete_one_user_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::OK, "User deleted successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[delete("/one_own")]
async fn delete_one_user_by_id_own(data: web::Data<AppState>, claims: Claims) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = UserIdPayload { user_id: claims.sub.clone() };
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let delete_one_user_usecase = DeleteOneUserByIdUseCase::new(&user_payload, &data.users_repository);

    match delete_one_user_usecase.execute().await {
        Ok(user) => Ok(SuccessResponse::new(StatusCode::OK, "User deleted successfully", UserPresenterMapper::to_api(user)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}
