#[derive(Debug)]
pub enum Method {
    GET {
        file : String 
    },
    HEAD {
        file : String
    },
    POST {
        body : String,
        content_type : String
    },
    PUT,
    DELETE,
    CONNECT {
        URL : String
    },
    OPTIONS {
        URL : String
    },
    TRACE,
    PATCH
}

#[derive(Debug)]
pub enum ParserError {
    InvalidMethod,
    FileDoesNotExist,
    NotImplemented,
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

                match method.as_str() {
                    "GET"       => return Method::new_GET(request),
                    "HEAD"      => return Method::new_HEAD(request),
                    "POST"      => return Method::new_POST(request),
                    "PUT"       => return Method::new_PUT(request),
                    "DELETE"    => return Method::new_DELETE(request),
                    "CONNECT"   => return Method::new_CONNECT(request),
                    "OPTIONS"   => return Method::new_OPTIONS(request),
                    "TRACE"     => return Method::new_TRACE(request),
                    "PATCH"     => return Method::new_PATCH(request),
                    _           => return Result::Err(ParserError::InvalidMethod),
                }
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

    fn new_HEAD(request : Vec<&str>) -> Result<Method, ParserError> {
        let start_line : Vec<&str> = request[0].clone().split_whitespace().collect();

        if start_line.len() < 3 {
            return Result::Err(ParserError::InvalidMethod);
        }

        let file_name : &str = start_line[1];

        Result::Ok(Method::HEAD{ file : file_name.to_string()})
    }

    fn new_POST(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn new_PUT(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn new_DELETE(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn new_CONNECT(request : Vec<&str>) -> Result<Method, ParserError> {
        let start_line : Vec<&str> = request[0].clone().split_whitespace().collect();

        if start_line.len() < 3 {
            return Result::Err(ParserError::InvalidMethod);
        }

        let URL_name : &str = start_line[1];

        Result::Ok(Method::CONNECT{ URL : URL_name.to_string()})
    }

    fn new_OPTIONS(request : Vec<&str>) -> Result<Method, ParserError> {
        let start_line : Vec<&str> = request[0].clone().split_whitespace().collect();

        if start_line.len() < 3 {
            return Result::Err(ParserError::InvalidMethod);
        }

        let URL_name : &str = start_line[1];

        Result::Ok(Method::OPTIONS{ URL : URL_name.to_string()})
    }

    fn new_TRACE(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn new_PATCH(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn get_type<'a>(request: &'a str) -> Option<&'a str> {
        request.split_whitespace().next()
    }
}