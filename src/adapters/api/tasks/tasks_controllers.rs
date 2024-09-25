use crate::{
    adapters::api::{
        shared::{app_state::AppState, error_presenter::ErrorResponse, success_presenter::SuccessResponse},
        tasks::{
            tasks_mappers::*,
            tasks_payloads::{TaskCreatePayload, TaskDataPayload, TaskUpdatePayload},
            tasks_presenters::TaskAllPresenter,
        },
    },
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{
            interfaces::AbstractUseCase,
            task::{
                delete_one_task_by_id_usecase::DeleteOneTaskByIdUseCase, get_all_tasks_usecase::GetAllTasksUseCase, get_one_task_by_id_usecase::GetOneTaskByIdUseCase,
                post_one_task_usecase::PostOneTaskUseCase, update_one_task_usecase::UpdateOneTaskUseCase,
            },
        },
        utils::access_control::{auth_usecase::AuthCheckUseCase, extractors::claims::Claims},
    },
    domain::{error::ApiError, task_entity::*},
};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use reqwest::StatusCode;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_one_task)
        .service(post_one_task_own)
        .service(update_one_task)
        .service(update_one_task_own)
        .service(get_all_tasks)
        .service(get_all_tasks_by_user_id)
        .service(get_all_tasks_by_user_id_own)
        .service(get_one_task_by_id)
        .service(get_one_task_by_id_own)
        .service(delete_one_task_by_id)
        .service(delete_one_task_by_id_own);
}

#[post("/one")]
async fn post_one_task(data: web::Data<AppState>, claims: Claims, path: web::Json<TaskCreatePayload>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let task_payload = path.into_inner();
    let post_one_task_usecase = PostOneTaskUseCase::new(&task_payload, &data.tasks_repository);

    match post_one_task_usecase.execute().await {
        Ok(task) => Ok(SuccessResponse::new(StatusCode::CREATED, "Task created successfully", TaskPresenterMapper::to_api(task)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[post("/one_own")]
async fn post_one_task_own(data: web::Data<AppState>, claims: Claims, path: web::Json<TaskCreatePayload>) -> Result<HttpResponse, ErrorResponse> {
    let mut task_payload = path.into_inner();
    task_payload.user_id = Some(claims.sub.clone());
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let post_one_task_usecase = PostOneTaskUseCase::new(&task_payload, &data.tasks_repository);

    match post_one_task_usecase.execute().await {
        Ok(task) => Ok(SuccessResponse::new(StatusCode::CREATED, "Task created successfully", TaskPresenterMapper::to_api(task)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/all")]
async fn get_all_tasks(data: web::Data<AppState>, claims: Claims, path: Option<web::Json<TaskDataPayload>>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let task_payload = path.unwrap_or_else(|| web::Json(TaskDataPayload::default())).into_inner();
    let get_all_tasks_usecase = GetAllTasksUseCase::new(&task_payload, &data.tasks_repository);
    let tasks: Result<Vec<TaskAllEntity>, ApiError> = get_all_tasks_usecase.execute().await;

    match tasks {
        Ok(task) => Ok(SuccessResponse::new(
            StatusCode::OK,
            "Tasks retrieved successfully",
            task.into_iter().map(TaskAllPresenterMapper::to_api).collect::<Vec<TaskAllPresenter>>(),
        )
        .to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/all_by_user_id")]
async fn get_all_tasks_by_user_id(data: web::Data<AppState>, claims: Claims, path: web::Json<TaskDataPayload>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let task_payload = path.into_inner();
    let get_all_tasks_usecase = GetAllTasksUseCase::new(&task_payload, &data.tasks_repository);
    let tasks: Result<Vec<TaskAllEntity>, ApiError> = get_all_tasks_usecase.execute().await;

    match tasks {
        Ok(task) => Ok(SuccessResponse::new(
            StatusCode::OK,
            "Tasks retrieved successfully",
            task.into_iter().map(TaskAllPresenterMapper::to_api).collect::<Vec<TaskAllPresenter>>(),
        )
        .to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/all_by_user_id_own")]
async fn get_all_tasks_by_user_id_own(data: web::Data<AppState>, claims: Claims, path: Option<web::Json<TaskDataPayload>>) -> Result<HttpResponse, ErrorResponse> {
    let mut task_payload = path.unwrap_or_else(|| web::Json(TaskDataPayload::default())).into_inner();
    task_payload.user_id = Some(claims.sub.clone());
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let get_all_tasks_usecase = GetAllTasksUseCase::new(&task_payload, &data.tasks_repository);
    let tasks: Result<Vec<TaskAllEntity>, ApiError> = get_all_tasks_usecase.execute().await;

    match tasks {
        Ok(task) => Ok(SuccessResponse::new(
            StatusCode::OK,
            "Tasks retrieved successfully",
            task.into_iter().map(TaskAllPresenterMapper::to_api).collect::<Vec<TaskAllPresenter>>(),
        )
        .to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[patch("/one")]
async fn update_one_task(data: web::Data<AppState>, claims: Claims, path: web::Json<TaskUpdatePayload>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_admin(claims)?;
    let task_payload = path.into_inner();
    let update_one_task_usecase = UpdateOneTaskUseCase::new(&task_payload, &data.tasks_repository);

    match update_one_task_usecase.execute().await {
        Ok(task) => Ok(SuccessResponse::new(StatusCode::OK, "Task updated successfully", TaskPresenterMapper::to_api(task)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[patch("/one_own")]
async fn update_one_task_own(data: web::Data<AppState>, claims: Claims, path: web::Json<TaskUpdatePayload>) -> Result<HttpResponse, ErrorResponse> {
    let mut task_payload = path.into_inner();
    task_payload.user_id = Some(claims.sub.clone());
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let update_one_task_usecase = UpdateOneTaskUseCase::new(&task_payload, &data.tasks_repository);

    match update_one_task_usecase.execute().await {
        Ok(task) => Ok(SuccessResponse::new(StatusCode::OK, "Task updated successfully", TaskPresenterMapper::to_api(task)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/one")]
async fn get_one_task_by_id(data: web::Data<AppState>, claims: Claims, path: web::Json<TaskDataPayload>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let task_payload = path.into_inner();
    let get_one_task_by_id_usecase = GetOneTaskByIdUseCase::new(&task_payload, &data.tasks_repository);

    match get_one_task_by_id_usecase.execute().await {
        Ok(task) => Ok(SuccessResponse::new(StatusCode::OK, "Task retrieved successfully", TaskPresenterMapper::to_api(task)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/one_own")]
async fn get_one_task_by_id_own(data: web::Data<AppState>, claims: Claims, path: Option<web::Json<TaskDataPayload>>) -> Result<HttpResponse, ErrorResponse> {
    let mut task_payload = path.unwrap_or_else(|| web::Json(TaskDataPayload::default())).into_inner();
    task_payload.user_id = Some(claims.sub.clone());
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let get_one_task_by_id_usecase = GetOneTaskByIdUseCase::new(&task_payload, &data.tasks_repository);

    match get_one_task_by_id_usecase.execute().await {
        Ok(task) => Ok(SuccessResponse::new(StatusCode::OK, "Task retrieved successfully", TaskPresenterMapper::to_api(task)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[delete("/one")]
async fn delete_one_task_by_id(data: web::Data<AppState>, claims: Claims, path: web::Json<TaskDataPayload>) -> Result<HttpResponse, ErrorResponse> {
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let task_payload = path.into_inner();
    let delete_one_task_usecase = DeleteOneTaskByIdUseCase::new(&task_payload, &data.tasks_repository);

    match delete_one_task_usecase.execute().await {
        Ok(task) => Ok(SuccessResponse::new(StatusCode::OK, "Task deleted successfully", TaskPresenterMapper::to_api(task)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[delete("/one_own")]
async fn delete_one_task_by_id_own(data: web::Data<AppState>, claims: Claims, path: Option<web::Json<TaskDataPayload>>) -> Result<HttpResponse, ErrorResponse> {
    let mut task_payload = path.unwrap_or_else(|| web::Json(TaskDataPayload::default())).into_inner();
    task_payload.user_id = Some(claims.sub.clone());
    AuthCheckUseCase::check_permission_up_to_user(claims)?;
    let delete_one_task_usecase = DeleteOneTaskByIdUseCase::new(&task_payload, &data.tasks_repository);

    match delete_one_task_usecase.execute().await {
        Ok(task) => Ok(SuccessResponse::new(StatusCode::OK, "Task deleted successfully", TaskPresenterMapper::to_api(task)).to_http_response()),
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}
