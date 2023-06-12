
# Table of Contents

1.  [API Handler](#org3276e41)
2.  [Key Features of the Crate](#org23ecc8e)
3.  [Example](#org678e36e)
    1.  [Before:](#orge43980f)
    2.  [After:](#orgdffc84d)
    3.  [What is the difference?](#org79ebc44)
4.  [Usage](#orga597a5b)
    1.  [Create an ApiError with Thiserror](#orgd176b49)
    2.  [Create ApiResponse](#org922cd5f)
    3.  [Create your argument and return types](#org83c72f6)
    4.  [Write your API's logic](#org925fb9d)


<a id="org3276e41"></a>

# API Handler

The API Handler crate provides a convenient way to handle API endpoints in Rust. It leverages Rust's macro system to automatically generate API handlers, simplifying error handling and data parsing.


<a id="org23ecc8e"></a>

# Key Features of the Crate

The key strength of this crate is the ease of error handling for API logic. When calling a function that returns a Result, simply use the ? operator; if the result is Ok, the value will be returned as is, while if the result is Err, the error handling defined in ResponseError will be automatically executed. Additionally, you only need to pass the argument type and return the response type, without needing to handle the HttpResponse directly.


<a id="org678e36e"></a>

# Example


<a id="orge43980f"></a>

## Before:

    #[derive(Serialize, Deserialize)]
    pub struct RequestRegister {
    	username: String,
    	password: String,
    	email: String,
    }
    
    
    pub async fn register(web::Json(info): web::Json<RequestRegister>, db_pool: web::Data<Pool<MySql>>) -> impl Responder {
    	let hashed_password = User::hash_password(&info.password);
    	let user_id = Uuid::new_v4();
    	let result = sqlx::query!(
    		"INSERT INTO user (user_id, username, email, password_hash) VALUES (?, ?, ?, ?)",
    		user_id.to_string(),
    		&info.username,
    		&info.email,
    		hashed_password
    	)
    		.execute(&**db_pool)
    		.await;
    
    	match result {
    		Ok(_) => HttpResponse::Created().finish(),
    		Err(e) => {
    			if let sqlx::Error::Database(db_err) = &e {
    				if db_err.message().contains("Duplicate entry") {
    					return HttpResponse::Conflict().body("Username or email already taken");
    				}
    			}
    			HttpResponse::InternalServerError().body(format!("Database error: {:?}", e))
    		},
    	}
    }


<a id="orgdffc84d"></a>

## After:

    #[derive(Serialize, Deserialize)]
    pub struct RequestRegister {
    	username: String,
    	password: String,
    	email: String,
    }
    
    
    #[derive(Serialize, Deserialize)]
    pub struct ResponseRegister;
    
    
    #[actix_api_handler::post_handler]
    pub async fn register(data:RequestRegister, db_pool: web::Data<Pool<MySql>>) -> Result<ResponseRegister,ApiError>{
    	let hashed_password = User::hash_password(&data.password);
    	let user_id = Uuid::new_v4();
    	let result = sqlx::query!(
    		"INSERT INTO user (user_id, username, email, password_hash) VALUES (?, ?, ?, ?)",
    		user_id.to_string(),
    		&data.username,
    		&data.email,
    		hashed_password
    	)
    		.execute(&**db_pool)
    		.await.map_err(|_| AuthError::InvalidCredentials)?;
    
    	Ok(ResponseRegister{})
    }


<a id="org79ebc44"></a>

## What is the difference?

-   Explicitness of Purpose:
    The use of #[actix<sub>api</sub><sub>handler</sub>::post<sub>handler</sub>] macro immediately signals the purpose of the function. In the "Before" example, it might not be immediately clear what the function is meant for. The annotation provides an instant context to other developers who may work on this code.

-   Better Error Handling:
    Error handling has been made significantly simpler and more manageable in the "After" code. Instead of manually handling each error type, Result<ResponseRegister, ApiError> abstracts away the error handling into ApiError, improving readability and maintainability.

-   Strong Typing:
    The creation of the ResponseRegister type makes the return type of the function clearer. In the "Before" example, there's no way of knowing what kind of response would be sent without reading through all of the code.

-   Cleaner Syntax:
    The "After" example makes use of the ? operator for error propagation, which allows us to avoid verbose match expressions. This makes the function much easier to read and understand.

-   Abstraction of HTTP Details:
    In the "Before" example, HTTP response details like the status code were explicitly handled in the code. In the "After" example, these are abstracted away. This makes the code cleaner and allows developers to focus on application logic instead of protocol details.

-   Better Consistency:
    By using a dedicated RequestRegister type for the input data, we ensure consistency in the way data is handled across different API endpoints.

-   Increased Maintainability:
    All of the above points contribute to make the "After" code more maintainable. It's easier to understand, cleaner, and will be more straightforward to modify in the future.


<a id="orga597a5b"></a>

# Usage


<a id="orgd176b49"></a>

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

This ResponseError lets you control the response on error.


<a id="org922cd5f"></a>

## Create ApiResponse

    #[derive(Serialize, Deserialize)]
    pub struct ApiResponse<T> {
        pub message: String,
        pub data: T,
    }

This becomes your response type.


<a id="org83c72f6"></a>

## Create your argument and return types

    #[derive(Deserialize, Serialize)]
    pub struct RequestSomeApi {
    }
    
    #[derive(Deserialize, Serialize)]
    pub struct ResponseSomeApi {
        id: String,
    }

Note that Serialize and Deserialize are required. While the argument type may be omitted, the return type is mandatory.


<a id="org925fb9d"></a>

## Write your API's logic

    #[api_handler::get_handler]
    pub async fn get_user_id(
        data: RequestGetSearchWord,
        db_pool: web::Data<Pool<MySql>>,
        req: HttpRequest,
    ) -> Result<ResponseGetSearchWord, ApiError> {
        let user= get_user(&db_pool, &req).await?;
        Ok(user.id)
    // any logic
    }

The data: RequestGetSearchWord argument is optional, but if present it must be named data and be the first argument. Other arguments can be reordered or omitted as needed. The return type must be Result<T, ApiError>.

