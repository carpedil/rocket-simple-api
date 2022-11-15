use migration::DbErr;
use rocket::Responder;



#[derive(Responder)]
#[response(status = 500,content_type = "json")]
pub(crate) struct ErrorResponder{
    pub message: String,
}


impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> Self {
        ErrorResponder { message: err.to_string() }
    }
}

impl  From<String> for ErrorResponder {
    fn from(string: String) -> Self {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> Self {
        ErrorResponder { message: str.to_owned() }
    }
}