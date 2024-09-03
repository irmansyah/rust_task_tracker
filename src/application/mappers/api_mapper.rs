use serde::Serialize;

pub trait ApiMapper<Entity, Presenter, Payload> {
    // Map an Entity to a Presenter
    fn to_api(entity: Entity) -> Presenter;

    // Map a Payload to an Entity
    fn to_entity(payload: Payload) -> Entity;
}

// pub struct PresenterMapper<T> {
//     pub code: i32,
//     pub message: String,
//     pub data: T,
// }


#[derive(Serialize)]
pub struct BaseResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

