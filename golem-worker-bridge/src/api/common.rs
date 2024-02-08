use std::fmt::Display;

use golem_common::model::TemplateId;
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, Object, Tags, Union};
use serde::{Deserialize, Serialize};

use crate::api_definition::MethodPattern;

#[derive(Tags)]
pub enum ApiTags {
    ApiDefinition,
    ApiDeployment,
    ApiDomain,
    ApiCertificate,
    Healthcheck,
}

#[derive(Union)]
#[oai(discriminator_name = "type", one_of = true)]
pub enum ErrorsBody {
    Messages(MessagesErrorsBody),
    Validation(ValidationErrorsBody),
}

#[derive(Object)]
pub struct MessagesErrorsBody {
    errors: Vec<String>,
}

#[derive(Object)]
pub struct ValidationErrorsBody {
    errors: Vec<RouteValidationError>,
}

#[derive(Object)]
pub struct ErrorBody {
    error: String,
}

#[derive(Object)]
pub struct MessageBody {
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Object)]
pub struct RouteValidationError {
    pub method: MethodPattern,
    pub path: String,
    pub template: TemplateId,
    pub detail: String,
}

#[derive(ApiResponse)]
pub enum ApiEndpointError {
    #[oai(status = 400)]
    BadRequest(Json<ErrorsBody>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorBody>),
    #[oai(status = 403)]
    LimitExceeded(Json<ErrorBody>),
    #[oai(status = 404)]
    NotFound(Json<MessageBody>),
    #[oai(status = 409)]
    AlreadyExists(Json<String>),
    #[oai(status = 500)]
    InternalError(Json<ErrorBody>),
}

impl ApiEndpointError {
    pub fn unauthorized<T: Display>(error: T) -> Self {
        Self::Unauthorized(Json(ErrorBody {
            error: error.to_string(),
        }))
    }

    pub fn internal<T: Display>(error: T) -> Self {
        Self::InternalError(Json(ErrorBody {
            error: error.to_string(),
        }))
    }

    pub fn bad_request<T: Display>(error: T) -> Self {
        Self::BadRequest(Json(ErrorsBody::Messages(MessagesErrorsBody {
            errors: vec![error.to_string()],
        })))
    }

    pub fn not_found<T: Display>(error: T) -> Self {
        Self::NotFound(Json(MessageBody {
            message: error.to_string(),
        }))
    }

    pub fn already_exists<T: Display>(error: T) -> Self {
        Self::AlreadyExists(Json(error.to_string()))
    }
}
