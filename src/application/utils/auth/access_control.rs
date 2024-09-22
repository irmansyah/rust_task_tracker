use std::collections::HashMap;
use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, Service}, Error, HttpResponse};
use futures::future::{ok, Ready};
use std::sync::Arc;
use actix_service::{Service as ActixService, ServiceFactory};

// Roles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    SuperAdmin,
    Admin,
    User,
    Public,  // New Public role
}

// Actions a role can perform
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    ReadOwn,
    ReadAny,
    CreateAny,
    UpdateAny,
    DeleteAny,
}

// Resources to be accessed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Resource {
    User,
    Task,
}

// Define permissions for each role
fn define_permissions() -> HashMap<Role, HashMap<Resource, Vec<Action>>> {
    let mut permissions = HashMap::new();

    permissions.insert(
        Role::SuperAdmin,
        vec![
            (Resource::User, vec![Action::ReadAny, Action::CreateAny, Action::UpdateAny, Action::DeleteAny]),
            (Resource::Task, vec![Action::ReadAny, Action::CreateAny, Action::UpdateAny, Action::DeleteAny]),
        ].into_iter().collect(),
    );

    permissions.insert(
        Role::Admin,
        vec![
            (Resource::User, vec![Action::ReadAny]),
            (Resource::Task, vec![Action::ReadAny, Action::CreateAny, Action::UpdateAny, Action::DeleteAny]),
        ].into_iter().collect(),
    );

    permissions.insert(
        Role::User,
        vec![
            (Resource::User, vec![Action::ReadOwn]),
            (Resource::Task, vec![Action::ReadOwn, Action::CreateAny, Action::UpdateAny]),
        ].into_iter().collect(),
    );

    permissions.insert(
        Role::Public,
        vec![
            (Resource::User, vec![Action::ReadAny]), // Public users can read any user info
        ].into_iter().collect(),
    );

    permissions
}

// Access control struct to check authorization
pub struct AccessControl {
    pub role: Role,
    pub action: Action,
    pub resource: Resource,
}

impl AccessControl {
    pub fn new(role: Role, action: Action, resource: Resource) -> Self {
        Self { role, action, resource }
    }

    fn is_authorized(&self, permissions: Arc<HashMap<Role, HashMap<Resource, Vec<Action>>>>) -> bool {
        if let Some(resource_permissions) = permissions.get(&self.role) {
            if let Some(actions) = resource_permissions.get(&self.resource) {
                return actions.contains(&self.action);
            }
        }
        false
    }
}

// Middleware for checking access
pub struct GrantAccessMiddleware {
    pub action: Action,
    pub resource: Resource,
    pub permissions: Arc<HashMap<Role, HashMap<Resource, Vec<Action>>>>,
}

impl GrantAccessMiddleware {
    pub fn new(action: Action, resource: Resource, permissions: Arc<HashMap<Role, HashMap<Resource, Vec<Action>>>>) -> Self {
        Self { action, resource, permissions }
    }
}

impl<S, B> Transform<S, ServiceRequest> for GrantAccessMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = GrantAccessMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(GrantAccessMiddlewareService {
            service,
            action: self.action.clone(),
            resource: self.resource.clone(),
            permissions: self.permissions.clone(),
        })
    }
}

// Inner middleware service
pub struct GrantAccessMiddlewareService<S> {
    service: S,
    action: Action,
    resource: Resource,
    permissions: Arc<HashMap<Role, HashMap<Resource, Vec<Action>>>>,
}

impl<S, B> ActixService<ServiceRequest> for GrantAccessMiddlewareService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Assume the user's role is attached to the request extensions
        let role = req.extensions().get::<Role>().cloned().unwrap_or(Role::Public);

        let access_control = AccessControl::new(role, self.action.clone(), self.resource.clone());

        if access_control.is_authorized(self.permissions.clone()) {
            self.service.call(req)
        } else {
            let (request, _) = req.into_parts();
            Box::pin(async move {
                Ok(ServiceResponse::new(
                    request,
                    HttpResponse::Unauthorized().json("Insufficient permissions"),
                ))
            })
        }
    }
}
