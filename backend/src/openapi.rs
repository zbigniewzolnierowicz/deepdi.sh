use serde_json::Value;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::modules::user::create_account,
        crate::modules::user::log_in,
        crate::modules::user::log_out,
        crate::modules::recipes::create_recipe,
        crate::modules::recipes::get_recipe,
        crate::modules::recipes::get_all_recipes,
        crate::modules::ingredients::create_ingredient,
        crate::modules::ingredients::get_ingredient,
    ),
    components(
        schemas(
            common::user::CreateNewUserDTO,
            common::user::LoginUserDTO,
            common::user::UserDataDTO,
            common::recipes::RecipeDTO,
            common::recipes::CreateRecipeDTO,
            common::recipes::CreateRecipeIngredient,
            common::ingredients::IngredientWithAmount,
            common::ingredients::IngredientDTO,
            common::ingredients::CreateIngredientDTO,
            common::error::ErrorMessage<Value>,
        )
    )
)]
pub struct ApiDoc;
