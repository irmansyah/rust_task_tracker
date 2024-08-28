use crate::{
    adapters::api::{
        shared::{app_state::AppState, error_presenter::ErrorResponse},
        tasks::{tasks_mappers::TaskPresenterMapper, tasks_presenters::TaskPresenter},
    },
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{get_all_tasks_usecase::GetAllTasksUseCase, get_one_task_by_id_usecase::GetOneTaskByIdUseCase, interfaces::AbstractUseCase, post_one_task_usecase::PostOneTaskUseCase},
    },
    domain::{error::ApiError, task_entity::TaskEntity},
};
use actix_web::{get, post, web, HttpResponse};

use super::tasks_payloads::TaskPayload;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_one_task).service(get_all_tasks).service(get_one_task_by_id);
}

#[post("/")]
async fn post_one_task(data: web::Data<AppState>, path: web::Json<TaskPayload>) -> Result<HttpResponse, ErrorResponse> {
    let task_payload = path.into_inner();
    let post_one_task_usecase = PostOneTaskUseCase::new(&task_payload, &data.tasks_repository);

    post_one_task_usecase
        .execute()
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|task| HttpResponse::Created().json(TaskPresenterMapper::to_api(task)))
}

#[get("/")]
async fn get_all_tasks(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_tasks_usecase = GetAllTasksUseCase::new(&data.tasks_repository);
    let tasks: Result<Vec<TaskEntity>, ApiError> = get_all_tasks_usecase.execute().await;

    tasks
        .map_err(ErrorResponse::map_io_error)
        .map(|tasks| HttpResponse::Ok().json(tasks.into_iter().map(TaskPresenterMapper::to_api).collect::<Vec<TaskPresenter>>()))
}

#[get("/{task_id}")]
async fn get_one_task_by_id(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse, ErrorResponse> {
    let task_id = path.into_inner().0;
    let get_one_task_by_id_usecase = GetOneTaskByIdUseCase::new(&task_id, &data.tasks_repository);
    let task = get_one_task_by_id_usecase.execute().await;

    task.map_err(ErrorResponse::map_io_error).map(|task| HttpResponse::Ok().json(TaskPresenterMapper::to_api(task)))
}
