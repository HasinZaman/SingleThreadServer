use super::HTTP::{
    Method::Method,
    Response::{
        ResponseStatusCode::ResponseStatusCode,
        Response::Response
    }
};
use super::FileReader;
use super::HTTP::Body::{ ContentType, Body };
use super::HTTP::Body::{ Application, Audio, Image, Multipart, Text, Video };

pub struct MethodLogic {
    pub get : Box<dyn Fn(Method)->Response>,
    pub head : Box<dyn Fn(Method)->Response>,
    pub post : Box<dyn Fn(Method)->Response>,
    pub put : Box<dyn Fn(Method)->Response>,
    pub delete : Box<dyn Fn(Method)->Response>,
    pub connect : Box<dyn Fn(Method)->Response>,
    pub option : Box<dyn Fn(Method)->Response>,
    pub trace : Box<dyn Fn(Method)->Response>,
    pub patch : Box<dyn Fn(Method)->Response>,
}

impl MethodLogic {
    pub fn default_not_allowed_logic() -> Box<dyn Fn(Method)->Response> {
        Box::new(
            |_request : Method | {
                Response {
                    status : ResponseStatusCode::MethodNotAllowed,
                    body : Option::None
                }
            }
        )
    }

    pub fn default_get_logic() -> Box<dyn Fn(Method)->Response> {
        Box::new(
            |request : Method | {
                match request {
                    Method::GET{ file } => {

                        let path_buf = FileReader::parse(&file);

                        let file_name = match &path_buf.file_name(){
                            Some(name) => name.to_str().unwrap(),
                            None => panic!("FileReader::parse does not return file name. {:?}", path_buf),
                        };

                        let body = FileReader::get_file_content(&path_buf).unwrap();

                        match file_name {
                            "404.html" => {
                                return Response {
                                    status : ResponseStatusCode::NotFound,
                                    body : Option::Some(
                                        Body {
                                            content_type: ContentType::Text{
                                                value: Text::html
                                            },
                                            content: body,
                                        }
                                    )
                                } 
                            },
                            _ => {
                                let content_type = path_buf.extension().unwrap();
                                let content_type = content_type.to_str().unwrap();

                                let content_type : ContentType = match content_type {
                                    "gif" => {
                                        ContentType::Image{
                                            value: Image::gif
                                        }
                                    },
                                    "jpg" | "jpeg" | "jpe" | "jfif" => {
                                        ContentType::Image{
                                            value: Image::jpeg
                                        }
                                    },
                                    "png" => {
                                        ContentType::Image{
                                            value: Image::png
                                        }
                                    },
                                    "tif" | "tiff" => {
                                        ContentType::Image{
                                            value: Image::tiff
                                        }
                                    },
                                    "css" => {
                                        ContentType::Text{
                                            value: Text::css
                                        }
                                    },
                                    "csv" => {
                                        ContentType::Text{
                                            value: Text::csv
                                        }
                                    },
                                    "html" => {
                                        ContentType::Text{
                                            value: Text::html
                                        }
                                    },
                                    ".js" => {
                                        ContentType::Text{
                                            value: Text::javascript
                                        }
                                    },
                                    "xml" => {
                                        ContentType::Text{
                                            value: Text::xml
                                        }
                                    },
                                    "mpeg" => {
                                        ContentType::Video{
                                            value: Video::mpeg
                                        }
                                    },
                                    "mp4" => {
                                        ContentType::Video{
                                            value: Video::mp4
                                        }
                                    },
                                    "webm" => {
                                        ContentType::Video{
                                            value: Video::webm
                                        }
                                    },
                                    _=>{
                                        return Response {
                                            status : ResponseStatusCode::NotFound,
                                            body : Option::None
                                        } 
                                    }
                                };



                                return Response {
                                    status : ResponseStatusCode::Ok,
                                    body : Option::Some(
                                        Body {
                                            content_type: content_type,
                                            content: body,
                                        }
                                    )
                                } 
                            }
                        }
                    },
                    _=> {
                        Response {
                            status : ResponseStatusCode::BadRequest,
                            body : Option::None
                        } 
                    }
                }
                
            }
        )
    }
}
