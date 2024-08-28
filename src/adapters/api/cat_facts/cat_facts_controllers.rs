use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::interfaces::AbstractUseCase;
use crate::application::usecases::post_one_cat_fact_usecase::PostOneCatFactUseCase;
use crate::{
    adapters::api::{
        cat_facts::{cat_facts_mappers::CatFactPresenterMapper, cat_facts_presenters::CatFactPresenter},
        shared::{app_state::AppState, error_presenter::ErrorResponse},
    },
    application::usecases::{get_all_cat_facts_usecase::GetAllCatFactsUseCase, get_one_random_cat_fact_usecase::GetOneRandomCatFactUseCase},
};
use actix_web::{get, post, web, HttpResponse};

use super::cat_facts_payloads::CatFactPayload;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_one_cat_fact).service(get_all_cat_facts).service(get_one_random_cat_fact);
}

#[post("/")]
async fn post_one_cat_fact(data: web::Data<AppState>, path: web::Json<CatFactPayload>) -> Result<HttpResponse, ErrorResponse> {
    let cat_fact_payload = path.into_inner();
    let post_one_cat_fact_usecase = PostOneCatFactUseCase::new(&cat_fact_payload, &data.cats_repository);

    post_one_cat_fact_usecase
        .execute()
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Created().json(CatFactPresenterMapper::to_api(fact)))
}

#[get("/")]
async fn get_all_cat_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&data.cats_repository);
    let cat_facts = get_all_cat_facts_usecase.execute().await;

    cat_facts
        .map_err(ErrorResponse::map_io_error)
        .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(CatFactPresenterMapper::to_api).collect::<Vec<CatFactPresenter>>()))
}

#[get("/random")]
async fn get_one_random_cat_fact(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&data.cats_repository);
    let cat_fact = get_one_random_cat_fact_usecase.execute().await;

    cat_fact
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(CatFactPresenterMapper::to_api(fact)))
}
