use std::collections::HashMap;
use std::error::Error;
use crate::validators::password_rules;
use crate::{
    app_state::SharedAppState,
    auth::security::hash_password,
    model::{
        repository::ModelRepository,
        users::{User, UserForCreate},
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum::extract::rejection::JsonRejection;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use validator::Validate;

/*
{
  "success": true,
  "message": "Customer created successfully.",
  "data": {
    "id": 1,
    "name": "John Doe",
    "email": "john.doe@example.com",
    "created_at": "2024-12-17T10:00:00Z",
    "updated_at": "2024-12-17T10:00:00Z"
  }
}


{
  "success": false,
  "message": "Validation failed.",
  "errors": {
    "name": [
      "The name field is required."
    ]
  }
}

{
  "success": false,
  "message": "An unexpected error occurred. Please try again later."
}

 */

#[derive(Serialize)]
#[serde(untagged)]
pub enum CreateUserResponse {
    Success(SuccessResponse),
    ValidationError(ValidationErrorResponse),
    GeneralError(GeneralErrorResponse),
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: User
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub success: bool,
    pub message: String,
    pub errors: HashMap<String, Vec<String>>
}

#[derive(Debug, Serialize)]
pub struct GeneralErrorResponse {
    pub success: bool,
    pub message: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 80))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 3, max = 64), custom(function = "password_rules"))]
    password: String,
}

#[debug_handler]
pub async fn create(
    State(state): State<SharedAppState>,
    payload: Result<Json<CreateUserRequest>, JsonRejection>,
) -> impl IntoResponse {

    // Step 1: Handle deserialization errors
    let payload = match payload {
        Ok(json) => json.0,
        Err(err) => {
            let mut errors = HashMap::new();

            // Attempt to extract the specific field name causing the error
            if let JsonRejection::JsonDataError(data_err) = &err {
                if let Some(serde_err) = data_err.source().and_then(|e| e.downcast_ref::<serde_json::Error>()) {
                    dbg!(&serde_err);
                    if let Some(field) = extract_missing_field(serde_err) {
                        errors.insert(field.to_string(), vec!["This field is required.".to_string()]);
                    } else {
                        errors.insert("general".to_string(), vec![format!("Invalid input: {}", err)]);
                    }
                } else {
                    errors.insert("general".to_string(), vec![format!("Invalid input: {}", err)]);
                }
            } else {
                errors.insert("general".to_string(), vec![format!("Invalid input: {}", err)]);
            }

            let response = CreateUserResponse::ValidationError(ValidationErrorResponse {
                    success: false,
                    message: "Invalid input.".to_string(),
                    errors
                });

                return (StatusCode::BAD_REQUEST, Json(response))
            }
        
    };

    // Step 2. Validate the payload
    if let Err(validation_errors) = payload.validate() {
        let errors = validation_errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                (
                    field.to_string(),
                    errors
                        .iter()
                        .map(|e| e.message
                            .clone()
                            .unwrap_or_else(||"Invalid value".into())
                            .to_string()
                        ).collect::<Vec<String>>(),
                )
            }).collect::<HashMap<String, Vec<String>>>();

        let response = CreateUserResponse::ValidationError(ValidationErrorResponse {
            success: false,
            message: "Validation failed.".to_string(),
            errors
        });
    }

    // Step 3. Hash the password
    let hashed_password = match hash_password(&payload.password, &state.app_key) {
        Ok(hash) => hash,
        Err(_) => {
            let response = CreateUserResponse::ValidationError(ValidationErrorResponse {
                    success: false,
                    message: "Failed to hash password.".to_string(),
                    errors: HashMap::new(),
                });
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    // Step 4. Save the user to the database
    let user_for_create = UserForCreate {
        name: payload.name,
        email: payload.email,
        hashed_password,
    };

    match User::create(&state.db_pool, user_for_create).await {
        Ok(user) => (
            StatusCode::CREATED,
            Json(CreateUserResponse::Success(SuccessResponse {
                    success: true,
                    message: "User created successfully.".to_string(),
                    data: user
                }
            )),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CreateUserResponse::GeneralError(GeneralErrorResponse {
                success: false,
                message: "An unexpected error occurred. Please try again later.".to_string(),
            })),
        ),
    }
}


fn extract_missing_field(err: &serde_json::Error) -> Option<String> {
    let error_message = err.to_string();
    dbg!(&error_message);
    let missing_field_prefix = "missing field `";
    
    if let Some(start) = error_message.find(missing_field_prefix) {
        let start_index = start + missing_field_prefix.len();
        if let Some(end_index) = error_message[start_index..].find('`') {
            return Some(error_message[start_index..start_index + end_index].to_string());
        }
    }
    
    None
}
