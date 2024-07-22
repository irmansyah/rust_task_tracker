use crate::{
    adapters::api::{
        dog_facts::{dog_facts_mappers::DogFactPresenterMapper, dog_facts_presenters::DogFactPresenter},
        shared::{app_state::AppState, error_presenter::ErrorResponse},
    },
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{get_all_dog_facts_usecase::GetAllDogFactsUseCase, get_one_dog_fact_by_id_usecase::GetOneDogFactByIdUseCase, interfaces::{AbstractPayloadUseCase, AbstractUseCase}, post_one_dog_fact_usecase::PostOneDogFactUseCase},
    },
    domain::{dog_fact_entity::DogFactEntity, error::ApiError},
};
use actix_web::{get, post, web, HttpResponse};

use super::dog_facts_payloads::DogFactPayload;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_one_dog_fact).service(get_all_dog_facts).service(get_one_dog_fact_by_id);
}

#[post("/")]
async fn post_one_dog_fact(data: web::Data<AppState>, payload: web::Json<DogFactPayload>) -> Result<HttpResponse, ErrorResponse> {
    let post_one_dog_fact_usecase = PostOneDogFactUseCase::new(&data.dogs_repository);
    let new_dog_fact = DogFactPayload::new(payload.fact_id.clone(), payload.fact.clone());

    post_one_dog_fact_usecase
        .execute(new_dog_fact)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Created().json(DogFactPresenterMapper::to_api(fact)))
}

#[get("/")]
async fn get_all_dog_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&data.dogs_repository);
    let dog_facts: Result<Vec<DogFactEntity>, ApiError> = get_all_dog_facts_usecase.execute().await;

    dog_facts
        .map_err(ErrorResponse::map_io_error)
        .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(DogFactPresenterMapper::to_api).collect::<Vec<DogFactPresenter>>()))
}

#[get("/{fact_id}")]
async fn get_one_dog_fact_by_id(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse, ErrorResponse> {
    let fact_id = path.into_inner().0;
    let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&fact_id, &data.dogs_repository);
    let dog_fact = get_one_dog_fact_by_id_usecase.execute().await;

    dog_fact
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(DogFactPresenterMapper::to_api(fact)))
}
