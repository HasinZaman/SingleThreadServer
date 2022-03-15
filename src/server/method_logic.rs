use std::collections::HashMap;
use std::str;

use super::file_reader;
use super::http::body::{Body, ContentType};
use super::http::body::{Image, Text, Video};
use super::http::{
    method::Method,
    response::{response::Response, responseStatusCode::ResponseStatusCode},
};
use super::setting::ServerSetting;

pub struct MethodLogic {
    pub get: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
    pub head: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
    pub post: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
    pub put: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
    pub delete: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
    pub connect: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
    pub option: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
    pub trace: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
    pub patch: Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response>,
}

impl MethodLogic {
    pub fn default_not_allowed_logic(
    ) -> Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response> {
        Box::new(
            |_request: Method,
             _server_settings: &ServerSetting,
             _meta_data: &HashMap<String, String>| Response {
                status: ResponseStatusCode::MethodNotAllowed,
                body: Option::None,
            },
        )
    }

    pub fn default_get_logic(
    ) -> Box<dyn Fn(Method, &ServerSetting, &HashMap<String, String>) -> Response> {
        Box::new(
            |request: Method,
             server_settings: &ServerSetting,
             meta_data: &HashMap<String, String>| match request {
                Method::Get { file } => {
                    let host: &str = match meta_data.get("host") {
                        Some(host) => host,
                        None => {
                            return Response {
                                status: ResponseStatusCode::NotFound,
                                body: Option::None,
                            }
                        }
                    };

                    let (host_path, allowed_extension) = match server_settings.paths.get(host) {
                        Some(host_path) => (&host_path.path, &host_path.allow),
                        None => {
                            return Response {
                                status: ResponseStatusCode::NotFound,
                                body: Option::None,
                            }
                        }
                    };

                    println!("{}", host_path);
                    let path_buf = match file_reader::parse(&file, &host_path, allowed_extension) {
                        Some(path_buf) => path_buf,
                        None => file_reader::parse("404.html", &host_path, &allowed_extension).unwrap(),
                    };

                    let file_name = match &path_buf.file_name() {
                        Some(name) => name.to_str().unwrap(),
                        None => panic!(
                            "FileReader::parse does not return file name. {:?}",
                            path_buf
                        ),
                    };

                    let body = file_reader::get_file_content_string(&path_buf).unwrap();


                    match file_name {
                        "404.html" => {
                            return Response {
                                status: ResponseStatusCode::NotFound,
                                body: Option::Some(Body {
                                    content_type: ContentType::Text(Text::html),
                                    content: body,
                                }),
                            }
                        }
                        _ => {
                            let content_type = path_buf.extension().unwrap();
                            let content_type = content_type.to_str().unwrap();

                            let content_type: ContentType = match content_type {
                                "gif" => ContentType::Image(Image::gif),
                                "jpg" | "jpeg" | "jpe" | "jfif" => ContentType::Image(Image::jpeg),
                                "png" => ContentType::Image(Image::png),
                                "tif" | "tiff" => ContentType::Image(Image::tiff),
                                "css" => ContentType::Text(Text::css),
                                "csv" => ContentType::Text(Text::csv),
                                "html" => ContentType::Text(Text::html),
                                "js" => ContentType::Text(Text::javascript),
                                "xml" => ContentType::Text(Text::xml),
                                "mpeg" => ContentType::Video(Video::mpeg),
                                "mp4" => ContentType::Video(Video::mp4),
                                "webm" => ContentType::Video(Video::webm),
                                _ => {
                                    return Response {
                                        status: ResponseStatusCode::NotFound,
                                        body: Option::None,
                                    }
                                }
                            };

                            return Response {
                                status: ResponseStatusCode::Ok,
                                body: Option::Some(Body {
                                    content_type: content_type,
                                    content: body,
                                }),
                            };
                        }
                    }
                }
                _ => Response {
                    status: ResponseStatusCode::BadRequest,
                    body: Option::None,
                },
            },
        )
    }
}
