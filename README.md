
# Table of Contents

1.  [What is this?](#org4ff9257)
2.  [Motivation](#org37797bc)
3.  [Benefit](#orgad7d39b)
4.  [Usage](#org2a8a1db)
    1.  [Create ApiResponse](#org8ce425c)
    2.  [Create an ApiError with Thiserror](#org100ecc8)
    3.  [Create your argument and return types](#org4e09a18)
    4.  [Write your API's logic](#org15ac7a8)
        1.  [query](#orga0d7430)
        2.  [path](#orge089baa)
        3.  [body](#orgf2f82a5)
    5.  [Add to your router](#org5c4d950)


<a id="org4ff9257"></a>

# What is this?

`api_type_handler` is a procedural macro designed to simplify the process of creating APIs with Actix-web, making your code cleaner and more maintainable.


<a id="org37797bc"></a>

# Motivation

Ordinarily, when creating APIs with Actix-web, functions carry arguments and return values as follows:

    pub async fn register(
    query: actix_web::web::Json<MyQuery>, 
    path: actix_web::web::Path<MyPath>,
    body: actix_web::web::Form<MyForm>,
    db_pool: actix_web::web::Data<Pool<MySql>>) -> impl Responder;

However, this approach has some limitations. Firstly, you must frequently use the `into_inner` method on path or query to access the underlying struct. Secondly, since the return type is not a Result, you have to write extensive error branching code, leading to deeply nested structures.

This crate was developed to overcome these hurdles. With this macro, you can now write your function as follows:

    pub async fn register(
    query: MyQuery, 
    path: MyPath,
    body: MyForm,
    db_pool: actix_web::web::Data<Pool<MySql>>) -> Result<MyResponse, ApiError>;


<a id="orgad7d39b"></a>

# Benefit

-   Simpler Arguments: Arguments are passed directly as structs, making the code cleaner and easier to read.
-   Error Handling: The return type is Result, which allows for more straightforward error handling within the function using the ? operator.
-   Custom Error Responses: You can control the behavior for error cases by implementing `actix_web::error::ResponseError` for ApiError.
-   Flexible Types: The function takes MyStruct as an argument and a Result-wrapped MyResponse as a return type, enabling the creation of unique argument and return types for each API.

By using the crate, you can achieve streamlined and maintainable code structures, reduce redundancy, and improve the overall readability and maintainability of your Actix-web applications.


<a id="org2a8a1db"></a>

# Usage


<a id="org8ce425c"></a>

## Create ApiResponse

    #[derive(Serialize, Deserialize)]
    pub struct ApiResponse<T> {
        pub message: String,
        pub data: T,
    }

This becomes your response type.


<a id="org100ecc8"></a>

## Create an ApiError with Thiserror

    #[derive(Error, Debug)]
    pub enum ApiError {
        #[error("Invalid credentials")]
        InvalidCredentials,
    // and so on
    }
    
    
    
    impl actix_web::error::ResponseError for ApiError {
        fn error_response(&self) -> actix_web::HttpResponse {
            use actix_web::http::StatusCode;
    
            let status_code = match self {
                ApiError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            };
    
            actix_web::HttpResponse::build(status_code).json(ApiResponse {
                message: self.to_string(),
                data: (),
            })
        }
    }

This is where you define the information to be returned to the user in the event of an error. Since the error type is enum, it is possible to define error messages and statuses without omissions.


<a id="org4e09a18"></a>

## Create your argument and return types

    #[derive(Deserialize, Serialize)]
    pub struct MyOwnQuery {
        user_name: String,
    }
    
    #[derive(Deserialize, Serialize)]
    pub struct MyResponse {
        id: String,
    }

Note that Serialize and Deserialize are required. While the argument type may be omitted, the return type is mandatory.


<a id="org15ac7a8"></a>

## Write your API's logic

    #[actix_type_handler::type_handler]
    pub async fn get_user_id(
        query: MyOwnQuery,
        db_pool: web::Data<Pool<MySql>>,
        req: HttpRequest,
    ) -> Result<MyResponse, ApiError> {
        let user= get_user(&db_pool, &req).await?;
        Ok(MyResponse{id:user.id})
    // any logic
    }

Special arguments are query, path, and body.


<a id="orga0d7430"></a>

### query

query is a reserved argument name to receive query parameters.
For example, it corresponds to a URL such as /api/search?s=123.


<a id="orge089baa"></a>

### path

path is a reserved argument name to receive path parameters.
For example, it corresponds to a URL such as /user/{`user_id`}/email.


<a id="orgf2f82a5"></a>

### body

body is a reserved argument name to receive body by POST and so on.

Note that these are not required arguments, but if they are taken as arguments, they must be named query, path, or body to be accepted.
The type name may be defined freely.


<a id="org5c4d950"></a>

## Add to your router

Please add `_api` as a postfix for your defined function name.

    .route("/api/auth/register", web::post().to(auth::register_api))

