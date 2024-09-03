use crate::{
    adapters::api::{
        shared::{app_state::AppState, error_presenter::ErrorResponse},
        tasks::{tasks_mappers::TaskPresenterMapper, tasks_presenters::TaskPresenter},
    },
    application::{
        mappers::api_mapper::{ApiMapper, BaseResponse},
        usecases::{
            delete_one_task_by_id_usecase::DeleteOneTaskByIdUseCase, get_all_tasks_usecase::GetAllTasksUseCase, get_one_task_by_id_usecase::GetOneTaskByIdUseCase,
            interfaces::AbstractUseCase, post_one_task_usecase::PostOneTaskUseCase, update_one_task_usecase::UpdateOneTaskUseCase,
        },
    },
    domain::{error::ApiError, task_entity::TaskEntity},
};
use actix_web::{delete, get, patch, post, web, HttpResponse};

use super::tasks_payloads::TaskPayload;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_one_task)
        .service(update_one_task)
        .service(get_all_tasks)
        .service(get_one_task_by_id)
        .service(delete_one_task_by_id);
}

#[post("/")]
async fn post_one_task(data: web::Data<AppState>, path: web::Json<TaskPayload>) -> Result<HttpResponse, ErrorResponse> {
    let task_payload = path.into_inner();
    let post_one_task_usecase = PostOneTaskUseCase::new(&task_payload, &data.tasks_repository);

    match post_one_task_usecase.execute().await {
        Ok(task) => {
            let response = BaseResponse {
                code: 201,
                message: "Task created successfully".to_string(),
                data: TaskPresenterMapper::to_api(task),
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[patch("/")]
async fn update_one_task(data: web::Data<AppState>, path: web::Json<TaskPayload>) -> Result<HttpResponse, ErrorResponse> {
    let task_payload = path.into_inner();
    let update_one_task_usecase = UpdateOneTaskUseCase::new(&task_payload, &data.tasks_repository);

    match update_one_task_usecase.execute().await {
        Ok(task) => {
            let response = BaseResponse {
                code: 200,
                message: "Task updated successfully".to_string(),
                data: TaskPresenterMapper::to_api(task),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/")]
async fn get_all_tasks(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_tasks_usecase = GetAllTasksUseCase::new(&data.tasks_repository);
    let tasks: Result<Vec<TaskEntity>, ApiError> = get_all_tasks_usecase.execute().await;

    match tasks {
        Ok(tasks) => {
            let response = BaseResponse {
                code: 200,
                message: "Task list retrieved successfully".to_string(),
                data: tasks.into_iter().map(TaskPresenterMapper::to_api).collect::<Vec<TaskPresenter>>(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[get("/{task_id}")]
async fn get_one_task_by_id(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse, ErrorResponse> {
    let task_id = path.into_inner().0;
    let get_one_task_by_id_usecase = GetOneTaskByIdUseCase::new(&task_id, &data.tasks_repository);

    match get_one_task_by_id_usecase.execute().await {
        Ok(task) => {
            let response = BaseResponse {
                code: 200,
                message: "Task retrieved successfully".to_string(),
                data: TaskPresenterMapper::to_api(task),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}

#[delete("/{task_id}")]
async fn delete_one_task_by_id(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse, ErrorResponse> {
    let task_id = path.into_inner().0;
    let delete_one_task_usecase = DeleteOneTaskByIdUseCase::new(&task_id, &data.tasks_repository);

    match delete_one_task_usecase.execute().await {
        Ok(task) => {
            let response = BaseResponse {
                code: 200,
                message: "Task deleted successfully".to_string(),
                data: TaskPresenterMapper::to_api(task),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(ErrorResponse::map_io_error(e)),
    }
}
