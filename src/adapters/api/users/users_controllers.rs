use crate::{
    adapters::api::{
        shared::{app_state::AppState, error_presenter::ErrorResponse},
        users::{
            users_mappers::UserPresenterMapper,
            users_payloads::{UserIdPayload, UserLoginPayload, UserRegisterPayload, UserUpdatePayload},
            users_presenters::UserPresenter,
        },
    },
    application::{
        mappers::api_mapper::{ApiMapper, BaseResponse},
        usecases::{
            interfaces::AbstractUseCase,
            user::{
                delete_one_user_by_id_usecase::DeleteOneUserByIdUseCase, get_all_users_usecase::GetAllUsersUseCase, get_one_user_by_id_usecase::GetOneUserByIdUseCase,
                login_user_usecase::LoginUserUseCase, register_user_usecase::RegisterUserUseCase, update_one_user_usecase::UpdateOneUserUseCase,
            },
        },
    },
    domain::{error::ApiError, user_entity::UserEntity},
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
    let register_user_usecase = RegisterUserUseCase::new(&user_payload, &data.users_repository);

    match register_user_usecase.execute().await {
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
async fn login_user(data: web::Data<AppState>, path: web::Json<UserLoginPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let login_user_usecase = LoginUserUseCase::new(&user_payload, &data.users_repository);

    match login_user_usecase.execute().await {
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
async fn update_one_user(data: web::Data<AppState>, path: web::Json<UserUpdatePayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let update_one_user_usecase = UpdateOneUserUseCase::new(&user_payload, &data.users_repository);

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

#[get("/all")]
async fn get_all_user(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_users_usecase = GetAllUsersUseCase::new(&data.users_repository);
    let user: Result<Vec<UserEntity>, ApiError> = get_all_users_usecase.execute().await;

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

#[get("/one")]
async fn get_one_user_by_id(data: web::Data<AppState>, path: web::Json<UserIdPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let get_one_user_by_id_usecase = GetOneUserByIdUseCase::new(&user_payload, &data.users_repository);

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

#[delete("/one")]
async fn delete_one_user_by_id(data: web::Data<AppState>, path: web::Json<UserIdPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_payload = path.into_inner();
    let delete_one_user_usecase = DeleteOneUserByIdUseCase::new(&user_payload, &data.users_repository);

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
