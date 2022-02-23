
#[derive(Debug)]
pub enum Method {
    GET {
        file : String 
    },
    HEAD {
        file : String
    },
    POST{
        body : String,
        contentType : String
    },
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

#[derive(Debug)]
pub enum ParserError {
    InvalidMethod,
    FileDoesNotExist,
}

impl Method {
    pub fn new(request_data : [u8; 1024]) -> Result<Method, ParserError> {
        let request : String = String::from_utf8_lossy(&request_data[..]).to_string();
        let request : Vec<&str> = request.split("\n").collect();

        if request.len() == 0 {
            return Result::Err(ParserError::InvalidMethod);
        }

        match Method::get_type(&request[0]){
            Some(method) => {
                let method : String = method.to_uppercase();

                if &method == "GET"{
                    return Method::new_GET(request);
                }

                return Result::Err(ParserError::InvalidMethod);
            },
            None => {
                return Result::Err(ParserError::InvalidMethod);
            },
        }
    }

    fn new_GET(request : Vec<&str>) -> Result<Method, ParserError> {
        let start_line : Vec<&str> = request[0].clone().split_whitespace().collect();

        if start_line.len() < 3 {
            return Result::Err(ParserError::InvalidMethod);
        }

        let file_name : &str = start_line[1];

        Result::Ok(Method::GET{ file : file_name.to_string()})
        
    }

    fn get_type<'a>(request: &'a str) -> Option<&'a str> {
        request.split_whitespace().next()
    }
}
