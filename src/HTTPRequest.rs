#[derive(Debug)]
pub enum Method {
    GET {
        file : String,
    },
    HEAD {
        file : String,
    },
    POST {
        file : String,
        body : Body,
    },
    PUT {
        file : String,
        body : Body,
    },
    DELETE {
        file : String,
        body : Option<Body>,
    },
    CONNECT {
        URL : String,
    },
    OPTIONS {
        URL : String,
    },
    TRACE,
    PATCH
}

#[derive(Debug)]
pub enum ParserError {
    InvalidMethod(Option<String>),
    FileDoesNotExist,
    NotImplemented,
}

#[derive(Debug)]
pub struct Body {
    pub content_type : HTTPBody::ContentType,
    pub content : String,
}

pub mod HTTPBody {
    use crate::HTTPRequest::ParserError;

    trait VariantName {
        fn get_variant(&self) -> String;
    }

    //https://www.geeksforgeeks.org/http-headers-content-type/
    pub enum ContentType {
        Application {value : Value::Application},
        Audio {value : Value::Audio},
        Image {value : Value::Image},
        Multipart {value : Value::Multipart},
        Text {value : Value::Text},
        Video {value : Value::Video},
    }
    
    impl ContentType {
        pub fn new( raw_str : &str ) -> Result<ContentType, ParserError> {
            let str_vec : Vec<&str> = raw_str.split("/").collect();

            if str_vec.len() < 2 {
                return Result::Err(ParserError::InvalidMethod(Option::Some(String::from("Start line must have more than 2 elements"))));
            }

            let type_raw : &str = str_vec[0];

            let value : &str = str_vec[1];

            match type_raw {
                "application" => {
                    let content_type_value = Value::parse_value::<Value::Application>(value)?;
                    let content_type = ContentType::Application{ value : content_type_value };
                    return Ok(content_type);
                }, 
                "audio" => {
                    let content_type_value = Value::parse_value::<Value::Audio>(value)?;
                    let content_type = ContentType::Audio{ value : content_type_value };
                    return Ok(content_type);
                },
                "image" => {
                    let content_type_value = Value::parse_value::<Value::Image>(value)?;
                    let content_type = ContentType::Image{ value : content_type_value };
                    return Ok(content_type);
                },
                "multipart" => {
                    let content_type_value = Value::parse_value::<Value::Multipart>(value)?;
                    let content_type = ContentType::Multipart{ value : content_type_value };
                    return Ok(content_type);
                },
                "text" => {
                    let content_type_value = Value::parse_value::<Value::Text>(value)?;
                    let content_type = ContentType::Text{ value : content_type_value };
                    return Ok(content_type);
                },
                "video" => {
                    let content_type_value = Value::parse_value::<Value::Video>(value)?;
                    let content_type = ContentType::Video{ value : content_type_value };
                    return Ok(content_type);
                },
                _ => return Result::Err(ParserError::InvalidMethod(Option::Some(String::from("Invalid type")))),
            }
        }
    }
    impl VariantName for ContentType {
        fn get_variant(&self) -> String{
            match &self {
                ContentType::Application { value } => return String::from("application"),
                ContentType::Audio { value } => return String::from("audio"),
                ContentType::Image { value } => return String::from("image"),
                ContentType::Multipart { value } => return String::from("multipart"),
                ContentType::Text { value } => return String::from("text"),
                ContentType::Video { value } => return String::from("video"),
                _ => panic!("Invalid variant type"),
            }
        }
    }
    impl ToString for ContentType {
        fn to_string(&self) -> String {
            match &self {
                ContentType::Application { value }  => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                ContentType::Audio { value }   => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                 ContentType::Image { value }   => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                 ContentType::Multipart { value }   => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                 ContentType::Text { value }   => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                 ContentType::Video { value } => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                _ => panic!("Not implemented variant"),
            }
            
        }
    }
    use std::fmt::{Debug, Formatter, Error};
    impl Debug for ContentType {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.debug_struct(&format!("ContentType: {}", self.to_string())).finish()
        }
    }

    pub mod Value{
        use crate::HTTPRequest::ParserError;

        pub trait Constructor<V>{
            fn new(value_raw : &str) -> Result<V, ParserError>;
        }

        pub enum Application {
            EDI_X12,
            EDIFACT,
            javascript,
            octet_stream,
            ogg,
            pdf,
            xhtml_xml,
            x_shockwave_flash,
            json,
            ld_json,
            xml,
            zip,
            x_www_form_urlencoded,
        }
        impl super::VariantName for Application {
            fn get_variant(&self) -> String{
                match &self {
                    Application::EDI_X12 => return String::from("EDI-X12"),
                    Application::EDIFACT => return String::from("EDIFACT"),
                    Application::javascript => return String::from("javascript"),
                    Application::octet_stream => return String::from("octet-stream"),
                    Application::ogg => return String::from("ogg"),
                    Application::pdf => return String::from("pdf"),
                    Application::xhtml_xml => return String::from("xhtml+xml"),
                    Application::x_shockwave_flash => return String::from("x-shockwave-flash"),
                    Application::json => return String::from("json"),
                    Application::ld_json => return String::from("ld+json"),
                    Application::xml => return String::from("xml"),
                    Application::zip => return String::from("zip"),
                    Application::x_www_form_urlencoded => return String::from("x-www-form-urlencoded"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }
        impl Constructor<Application> for Application{
            fn new(value_raw : &str) -> Result<Application, ParserError> {
                let value_paramater_vec : Vec<&str> = value_raw.split(";").collect();

                let value : &str = value_paramater_vec[0];
                match value {
                    "EDI-X12" => return Result::Ok(Application::EDI_X12),
                    "EDIFACT" => return Result::Ok(Application::EDIFACT),
                    "javascript" => return Result::Ok(Application::javascript),
                    "octet-stream" => return Result::Ok(Application::octet_stream),
                    "ogg" => return Result::Ok(Application::ogg),
                    "pdf" => return Result::Ok(Application::pdf),
                    "xhtml+xml" => return Result::Ok(Application::xhtml_xml),
                    "x-shockwave-flash" => return Result::Ok(Application::x_shockwave_flash),
                    "json" => return Result::Ok(Application::json),
                    "ld+json" => return Result::Ok(Application::ld_json),
                    "xml" => return Result::Ok(Application::xml),
                    "zip" => return Result::Ok(Application::zip),
                    "x-www-form-urlencoded" => return Result::Ok(Application::x_www_form_urlencoded),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Audio {
            mpeg,
            x_ms_wma,
            vnd_rn_realaudio,
            x_wav,
        }
        impl super::VariantName for Audio {
            fn get_variant(&self) -> String{
                match &self {
                    Audio::mpeg => return String::from("mpeg"),
                    Audio::x_ms_wma => return String::from("x-ms-wma"),
                    Audio::vnd_rn_realaudio => return String::from("vnd.rn-realaudio"),
                    Audio::x_wav => return String::from("x-wav"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }
        impl Constructor<Audio> for Audio{
            fn new(value_raw : &str) -> Result<Audio, ParserError> {
                let value_paramater_vec : Vec<&str> = value_raw.split(";").collect();

                let value : &str = value_paramater_vec[0];
                match value {
                    "mpeg" => return Result::Ok(Audio::mpeg),
                    "x-ms-wma" => return Result::Ok(Audio::x_ms_wma),
                    "vnd.rn-realaudio" => return Result::Ok(Audio::vnd_rn_realaudio),
                    "x-wav" => return Result::Ok(Audio::x_wav),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Image {
            gif,
            jpeg,
            png,
            tiff,
            vnd_microsoft_icon,
            x_icon,
            vnd_djvu,
            svg_xml,
        }
        impl super::VariantName for Image {
            fn get_variant(&self) -> String{
                match &self {
                    Image::gif => return String::from("gif"),
                    Image::jpeg => return String::from("jpeg"),
                    Image::png => return String::from("png"),
                    Image::tiff => return String::from("tiff"),
                    Image::vnd_microsoft_icon => return String::from("vnd.microsoft.icon"),
                    Image::x_icon => return String::from("x-icon"),
                    Image::vnd_djvu => return String::from("vnd.djvu"),
                    Image::svg_xml => return String::from("svg+xml"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }
        impl Constructor<Image> for Image{
            fn new(value_raw : &str) -> Result<Image, ParserError>{
                let value_paramater_vec : Vec<&str> = value_raw.split(";").collect();

                let value : &str = value_paramater_vec[0];
                match value {
                    "gif" => return Result::Ok(Image::gif),
                    "jpeg" => return Result::Ok(Image::jpeg),
                    "png" => return Result::Ok(Image::png),
                    "tiff" => return Result::Ok(Image::tiff),
                    "vnd.microsoft.icon" => return Result::Ok(Image::vnd_microsoft_icon),
                    "x-icon" => return Result::Ok(Image::x_icon),
                    "vnd.djvu" => return Result::Ok(Image::vnd_djvu),
                    "svg+xml" => return Result::Ok(Image::svg_xml),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Multipart {
            mixed,
            alternative,
            related,
            form_data {boundary : String},
        }
        impl super::VariantName for Multipart {
            fn get_variant(&self) -> String{
                match &self {
                    Multipart::mixed => String::from("mixed"),
                    Multipart::alternative => String::from("alternative"),
                    Multipart::related => String::from("related"),
                    Multipart::form_data { boundary } => String::from("form-data"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }
        impl Constructor<Multipart> for Multipart{
            fn new(value_raw : &str) -> Result<Multipart, ParserError>{
                let value_paramater_vec : Vec<&str> = value_raw.split(";").collect();

                let value : &str = value_paramater_vec[0];
                match value {
                    "mixed" => return Result::Ok(Multipart::mixed),
                    "alternative" => return Result::Ok(Multipart::alternative),
                    "related" => return Result::Ok(Multipart::related),
                    "form-data" => return Result::Ok(Multipart::form_data { boundary : String::from("")}),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Text {
            css,
            csv,
            html,
            javascript,
            plain,
            xml,
        }
        impl super::VariantName for Text {
            fn get_variant(&self) -> String{
                match &self {
                    Text::css => String::from("css"),
                    Text::csv => String::from("csv"),
                    Text::html => String::from("html"),
                    Text::javascript => String::from("javascript"),
                    Text::plain => String::from("plain"),
                    Text::xml => String::from("xml"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }
        impl Constructor<Text> for Text{
            fn new(value_raw : &str) -> Result<Text, ParserError>{
                let value_paramater_vec : Vec<&str> = value_raw.split(";").collect();

                let value : &str = value_paramater_vec[0];
                match value {
                    "css" => return Result::Ok(Text::css),
                    "csv" => return Result::Ok(Text::csv),
                    "html" => return Result::Ok(Text::html),
                    "javascript" => return Result::Ok(Text::javascript),
                    "plain" => return Result::Ok(Text::plain),
                    "xml" => return Result::Ok(Text::xml),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Video {
            mpeg,
            mp4,
            quicktime,
            x_ms_wmv,
            x_msvideo,
            x_flv,
            webm,
        }
        impl super::VariantName for Video {
            fn get_variant(&self) -> String{
                match &self {
                    Video::mpeg => String::from("mpeg"),
                    Video::mp4 => String::from("mp4"),
                    Video::quicktime => String::from("quicktime"),
                    Video::x_ms_wmv => String::from("x-ms-wmv"),
                    Video::x_msvideo => String::from("x-msvideo"),
                    Video::x_flv => String::from("x-flv"),
                    Video::webm => String::from("webm"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }
        impl Constructor<Video> for Video{
            fn new(value_raw : &str) -> Result<Video, ParserError>{
                let value_paramater_vec : Vec<&str> = value_raw.split(";").collect();

                let value : &str = value_paramater_vec[0];
                match value {
                    "mpeg" => return Result::Ok(Video::mpeg),
                    "mp4" => return Result::Ok(Video::mp4),
                    "quicktime" => return Result::Ok(Video::quicktime),
                    "x-ms-wmv" => return Result::Ok(Video::x_ms_wmv),
                    "x-msvideo" => return Result::Ok(Video::x_msvideo),
                    "x-flv" => return Result::Ok(Video::x_flv),
                    "webm" => return Result::Ok(Video::webm),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub fn parse_value<V>(value : &str) -> Result<V, ParserError> where V : Constructor<V> {
            V::new(value)
        }
    }
}

impl Method {
    pub fn new(request_data : [u8; 1024]) -> Result<Method, ParserError> {
        let request : String = String::from_utf8_lossy(&request_data[..]).to_string();
        let request : Vec<&str> = request.split("\n").collect();

        if request.len() == 0 {
            return Result::Err(ParserError::InvalidMethod(Option::Some(String::from("request must have more than 0 lines"))));
        }

        let method;
        let target;
        let version;
        
        match Method::get_start_line(&request[0]) {
            Ok(tmp) => {
                method = tmp.0;
                target = tmp.1;
                version = tmp.2;
            },
            Err(tmp) => return Result::Err(tmp),
        }

        let method : String = method.to_uppercase();

        match method.as_str() {
            "GET"       => return Result::Ok(Method::GET{ file : target.to_string() }),
            "HEAD"      => return Result::Ok(Method::HEAD{ file : target.to_string() }),
            "POST"      => return Result::Ok(Method::POST{ file : target.to_string(), body : Method::get_body(&request)?}),
            "PUT"       => return Result::Ok(Method::PUT{ file : target.to_string(), body : Method::get_body(&request)?}),
            "DELETE"    => return Result::Err(ParserError::NotImplemented),//return Result::Ok(Method::DELETE{ file : target.to_string(), body : Method::get_body(&request)?}),
            // DELETE { file : String, body : Option<Body>, },
            "CONNECT"   => return Result::Ok(Method::CONNECT{ URL : target.to_string() }),
            "OPTIONS"   => return Result::Ok(Method::OPTIONS{ URL : target.to_string() }),
            "TRACE"     => return Result::Err(ParserError::NotImplemented),
            "PATCH"     => return Result::Err(ParserError::NotImplemented),
            _           => return Result::Err(ParserError::InvalidMethod(Option::Some(String::from(format!("{} is an invalid method name", method.as_str()))))),
        }
    }

    fn get_start_line<'a>(start_line: &'a str) -> Result<(&'a str, &'a str, &'a str), ParserError> {
        let mut start_line = start_line.split_whitespace();
        
        let method : &str = start_line.next().ok_or(ParserError::InvalidMethod(Option::None))?;
        let target : &str = start_line.next().ok_or(ParserError::InvalidMethod(Option::None))?;
        let version : &str = start_line.next().ok_or(ParserError::InvalidMethod(Option::None))?;

        Result::Ok((method, target, version))
    }

    fn get_body<'a>(request : &Vec<&str>) -> Result<Body, ParserError> {
        let mut content_type : Option<HTTPBody::ContentType> = Option::None;
        let mut boundary_index : Option<usize> = Option::None;

        //get content type
        for i1 in 1..request.len() {
            if 13 <= request[i1].len() {
                let content_type_result = HTTPBody::ContentType::new(request[i1]);

                match content_type_result {
                    Ok( tmp ) => {
                        content_type = Option::Some(tmp);
                    },
                        let content_type_result = HTTPBody::ContentType::new(value.trim_start());
                            return Result::Err(ParserError::InvalidMethod(Option::Some(String::from("No content type before beginning of body"))))
                }
            }
            else if 0 == request[i1].len() {
                if content_type.is_none() {
                    return Result::Err(ParserError::InvalidMethod)
                }

                boundary_index = Option::Some(i1);
                break;
            }
            println!("{}", request[i1].to_string());
        };

        let mut body : String = String::from("");

        match boundary_index {
            Some(tmp) => {
                for i1 in tmp..request.len() {
                    body.push_str(request[i1]);
                }
            },
            None => panic!("boundary_index is invalid type (None)"),
        }
        

        match content_type {
            Some( tmp ) => return Result::Ok(Body { content_type : tmp, content : body }),
            None => panic!("content_type is invalid type (None)"),
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match &self {
            Method::GET { file } => {
                return String::from(format!("GET file={}", file));
            },
            Method::HEAD { file } => {
                return String::from(format!("Head file={}", file));
            },
            Method::POST { file, body } => {
                return String::from(format!("POST file={}", file));
            },
            Method::PUT { file, body } => {
                return String::from(format!("PUT file={}", file));
            },
            Method::DELETE { file, body } => {
                return String::from(format!("DELETE file={}", file));
            },
            Method::CONNECT { URL } => {
                return String::from(format!("CONNECT URL={}", URL));
            },
            Method::OPTIONS { URL } => {
                return String::from(format!("OPTIONS URL={}", URL));
            },
            Method::TRACE => {
                return String::from("TRACE");
            },
            Method::PATCH => {
                return String::from("PATCH");
            }
            _ => panic!("Not implemented variant"),
        }
    }
}
#[cfg(test)]
mod tests {
    mod http_body_enum_test {
        
        mod enum_to_string_test{
            use crate::HTTPRequest::HTTPBody::*;

            #[test]
            fn application_test(){
                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::EDI_X12
                    };
                    assert_eq!(actual.to_string(), "application/EDI-X12");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::EDIFACT
                    };
                    assert_eq!(actual.to_string(), "application/EDIFACT");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::javascript
                    };
                    assert_eq!(actual.to_string(), "application/javascript");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::octet_stream
                    };
                    assert_eq!(actual.to_string(), "application/octet-stream");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::ogg
                    };
                    assert_eq!(actual.to_string(), "application/ogg");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::pdf
                    };
                    assert_eq!(actual.to_string(), "application/pdf");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::xhtml_xml
                    };
                    assert_eq!(actual.to_string(), "application/xhtml+xml");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::x_shockwave_flash
                    };
                    assert_eq!(actual.to_string(), "application/x-shockwave-flash");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::json
                    };
                    assert_eq!(actual.to_string(), "application/json");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::ld_json
                    };
                    assert_eq!(actual.to_string(), "application/ld+json");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::xml
                    };
                    assert_eq!(actual.to_string(), "application/xml");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::zip
                    };
                    assert_eq!(actual.to_string(), "application/zip");
                }

                {
                    let actual : ContentType = ContentType::Application {
                        value : Value::Application::x_www_form_urlencoded
                    };
                    assert_eq!(actual.to_string(), "application/x-www-form-urlencoded");
                }
            }

            #[test]
            fn audio_test(){
                {
                    let actual : ContentType = ContentType::Audio {
                        value : Value::Audio::mpeg
                    };
                    assert_eq!(actual.to_string(), "audio/mpeg");
                }
                
                {
                    let actual : ContentType = ContentType::Audio {
                        value : Value::Audio::x_ms_wma
                    };
                    assert_eq!(actual.to_string(), "audio/x-ms-wma");
                }
                
                {
                    let actual : ContentType = ContentType::Audio {
                        value : Value::Audio::vnd_rn_realaudio
                    };
                    assert_eq!(actual.to_string(), "audio/vnd.rn-realaudio");
                }
                
                {
                    let actual : ContentType = ContentType::Audio {
                        value : Value::Audio::x_wav
                    };
                    assert_eq!(actual.to_string(), "audio/x-wav");
                }
            }

            #[test]
            fn image_test(){
                {
                    let actual : ContentType = ContentType::Image {
                        value : Value::Image::gif
                    };
                    assert_eq!(actual.to_string(), "image/gif");
                }
                
                {
                    let actual : ContentType = ContentType::Image {
                        value : Value::Image::jpeg
                    };
                    assert_eq!(actual.to_string(), "image/jpeg");
                }
                
                {
                    let actual : ContentType = ContentType::Image {
                        value : Value::Image::png
                    };
                    assert_eq!(actual.to_string(), "image/png");
                }
                
                {
                    let actual : ContentType = ContentType::Image {
                        value : Value::Image::tiff
                    };
                    assert_eq!(actual.to_string(), "image/tiff");
                }
                
                {
                    let actual : ContentType = ContentType::Image {
                        value : Value::Image::vnd_microsoft_icon
                    };
                    assert_eq!(actual.to_string(), "image/vnd.microsoft.icon");
                }
                
                {
                    let actual : ContentType = ContentType::Image {
                        value : Value::Image::x_icon
                    };
                    assert_eq!(actual.to_string(), "image/x-icon");
                }
                
                {
                    let actual : ContentType = ContentType::Image {
                        value : Value::Image::vnd_djvu
                    };
                    assert_eq!(actual.to_string(), "image/vnd.djvu");
                }
                
                {
                    let actual : ContentType = ContentType::Image {
                        value : Value::Image::svg_xml
                    };
                    assert_eq!(actual.to_string(), "image/svg+xml");
                }
            }

            #[test]
            fn multipart_test(){
                {
                    let actual : ContentType = ContentType::Multipart {
                        value : Value::Multipart::mixed
                    };
                    assert_eq!(actual.to_string(), "multipart/mixed");
                }

                {
                    let actual : ContentType = ContentType::Multipart {
                        value : Value::Multipart::alternative
                    };
                    assert_eq!(actual.to_string(), "multipart/alternative");
                }

                {
                    let actual : ContentType = ContentType::Multipart {
                        value : Value::Multipart::related
                    };
                    assert_eq!(actual.to_string(), "multipart/related");
                }

                {
                    let actual : ContentType = ContentType::Multipart {
                        value : Value::Multipart::form_data { boundary : String::from("")}
                    };
                    assert_eq!(actual.to_string(), "multipart/form-data");
                }
            }

            #[test]
            fn text_test(){
                {
                    let actual : ContentType = ContentType::Text {
                        value : Value::Text::css
                    };
                    assert_eq!(actual.to_string(), "text/css");
                }
                
                {
                    let actual : ContentType = ContentType::Text {
                        value : Value::Text::csv
                    };
                    assert_eq!(actual.to_string(), "text/csv");
                }
                
                {
                    let actual : ContentType = ContentType::Text {
                        value : Value::Text::html
                    };
                    assert_eq!(actual.to_string(), "text/html");
                }
                
                {
                    let actual : ContentType = ContentType::Text {
                        value : Value::Text::javascript
                    };
                    assert_eq!(actual.to_string(), "text/javascript");
                }
                
                {
                    let actual : ContentType = ContentType::Text {
                        value : Value::Text::plain
                    };
                    assert_eq!(actual.to_string(), "text/plain");
                }
                
                {
                    let actual : ContentType = ContentType::Text {
                        value : Value::Text::xml
                    };
                    assert_eq!(actual.to_string(), "text/xml");
                }
            }

            #[test]
            fn video_test(){
                {
                    let actual : ContentType = ContentType::Video {
                        value : Value::Video::mpeg
                    };
                    assert_eq!(actual.to_string(), "video/mpeg");
                }
                
                {
                    let actual : ContentType = ContentType::Video {
                        value : Value::Video::mp4
                    };
                    assert_eq!(actual.to_string(), "video/mp4");
                }
                
                {
                    let actual : ContentType = ContentType::Video {
                        value : Value::Video::quicktime
                    };
                    assert_eq!(actual.to_string(), "video/quicktime");
                }
                
                {
                    let actual : ContentType = ContentType::Video {
                        value : Value::Video::x_ms_wmv
                    };
                    assert_eq!(actual.to_string(), "video/x-ms-wmv");
                }
                
                {
                    let actual : ContentType = ContentType::Video {
                        value : Value::Video::x_msvideo
                    };
                    assert_eq!(actual.to_string(), "video/x-msvideo");
                }
                
                {
                    let actual : ContentType = ContentType::Video {
                        value : Value::Video::x_flv
                    };
                    assert_eq!(actual.to_string(), "video/x-flv");
                }
                
                {
                    let actual : ContentType = ContentType::Video {
                        value : Value::Video::webm
                    };
                    assert_eq!(actual.to_string(), "video/webm");
                }
            }
        }

        mod string_to_enum_test {
            use crate::HTTPRequest::HTTPBody::*;

            fn string_to_enum_test(input_str : &str, expected : ContentType) {
                let actual = ContentType::new(input_str);

                match actual {
                    Ok(actual) => {
                        assert_eq!(actual.to_string(), expected.to_string());
                    },
                    _ => panic!("Parse Error")
                }
            }

            #[test]
            fn application_EDI_X12_test(){
                string_to_enum_test(
                    "application/EDI-X12",
                    ContentType::Application { value : Value::Application::EDI_X12 }
                );
            }

            #[test]
            fn application_EDIFACT_test(){
                string_to_enum_test(
                    "application/EDIFACT",
                    ContentType::Application { value : Value::Application::EDIFACT }
                );
            }

            #[test]
            fn application_javascript_test(){
                string_to_enum_test(
                    "application/javascript",
                    ContentType::Application { value : Value::Application::javascript }
                );
            }

            #[test]
            fn application_octet_stream_test(){
                string_to_enum_test(
                    "application/octet-stream",
                    ContentType::Application { value : Value::Application::octet_stream }
                );
            }

            #[test]
            fn application_ogg_test(){
                string_to_enum_test(
                    "application/ogg",
                    ContentType::Application { value : Value::Application::ogg }
                );
            }

            #[test]
            fn application_pdf_test(){
                string_to_enum_test(
                    "application/pdf",
                    ContentType::Application { value : Value::Application::pdf }
                );
            }

            #[test]
            fn application_xhtml_xml_test(){
                string_to_enum_test(
                    "application/xhtml+xml",
                    ContentType::Application { value : Value::Application::xhtml_xml }
                );
            }
            
            #[test]
            fn application_x_shockwave_flash_test(){
                string_to_enum_test(
                    "application/x-shockwave-flash",
                    ContentType::Application { value : Value::Application::x_shockwave_flash }
                );
            }
            
            #[test]
            fn application_json_test(){
                string_to_enum_test(
                    "application/json",
                    ContentType::Application { value : Value::Application::json }
                );
            }
            
            #[test]
            fn application_ld_json_test(){
                string_to_enum_test(
                    "application/ld+json",
                    ContentType::Application { value : Value::Application::ld_json }
                );
            }
            
            #[test]
            fn application_zip_test(){
                string_to_enum_test(
                    "application/zip",
                    ContentType::Application { value : Value::Application::zip }
                );
            }
            
            #[test]
            fn application_x_www_form_urlencoded_test(){
                string_to_enum_test(
                    "application/x-www-form-urlencoded",
                    ContentType::Application { value : Value::Application::x_www_form_urlencoded }
                );
            }

            #[test]
            fn audio_mpeg_test(){
                string_to_enum_test(
                    "audio/mpeg",
                    ContentType::Audio { value : Value::Audio::mpeg }
                );
            }

            #[test]
            fn audio_vnd_rn_realaudio_test(){
                string_to_enum_test(
                    "audio/vnd.rn-realaudio",
                    ContentType::Audio { value : Value::Audio::vnd_rn_realaudio }
                );
            }

            #[test]
            fn audio_x_wav_test(){
                string_to_enum_test(
                    "audio/x-wav",
                    ContentType::Audio { value : Value::Audio::x_wav }
                );
            }

            #[test]
            fn image_gif_test(){
                string_to_enum_test(
                    "image/gif",
                    ContentType::Image { value : Value::Image::gif }
                );
            }

            #[test]
            fn image_jpeg_test(){
                string_to_enum_test(
                    "image/jpeg",
                    ContentType::Image { value : Value::Image::jpeg }
                );
            }

            #[test]
            fn image_png_test(){
                string_to_enum_test(
                    "image/png",
                    ContentType::Image { value : Value::Image::png }
                );
            }

            #[test]
            fn image_tiff_test(){
                string_to_enum_test(
                    "image/tiff",
                    ContentType::Image { value : Value::Image::tiff }
                );
            }

            #[test]
            fn image_vnd_microsoft_icon_test(){
                string_to_enum_test(
                    "image/vnd.microsoft.icon",
                    ContentType::Image { value : Value::Image::vnd_microsoft_icon }
                );
            }

            #[test]
            fn image_x_icon_test(){
                string_to_enum_test(
                    "image/x-icon",
                    ContentType::Image { value : Value::Image::x_icon }
                );
            }

            #[test]
            fn image_vnd_djvu_test(){
                string_to_enum_test(
                    "image/vnd.djvu",
                    ContentType::Image { value : Value::Image::vnd_djvu }
                );
            }

            #[test]
            fn image_svg_xml_test(){
                string_to_enum_test(
                    "image/svg+xml",
                    ContentType::Image { value : Value::Image::svg_xml }
                );
            }

            #[test]
            fn multipart_mixed_test(){
                string_to_enum_test(
                    "multipart/mixed",
                    ContentType::Multipart { value : Value::Multipart::mixed }
                );
            }

            #[test]
            fn multipart_alternative_test(){
                string_to_enum_test(
                    "multipart/alternative",
                    ContentType::Multipart { value : Value::Multipart::alternative }
                );
            }

            #[test]
            fn multipart_related_test(){
                string_to_enum_test(
                    "multipart/related",
                    ContentType::Multipart { value : Value::Multipart::related }
                );
            }

            #[test]
            fn multipart_form_data_test(){
                string_to_enum_test(
                    "multipart/form-data",
                    ContentType::Multipart { value : Value::Multipart::form_data { boundary : String::from("")} }
                );
            }

            #[test]
            fn text_css_test(){
                string_to_enum_test(
                    "text/css",
                    ContentType::Text { value : Value::Text::css }
                );
            }

            #[test]
            fn text_csv_test(){
                string_to_enum_test(
                    "text/csv",
                    ContentType::Text { value : Value::Text::csv }
                );
            }

            #[test]
            fn text_html_test(){
                string_to_enum_test(
                    "text/html",
                    ContentType::Text { value : Value::Text::html }
                );
            }

            #[test]
            fn text_javascript_test(){
                string_to_enum_test(
                    "text/javascript",
                    ContentType::Text { value : Value::Text::javascript }
                );
            }

            #[test]
            fn text_plain_test(){
                string_to_enum_test(
                    "text/plain",
                    ContentType::Text { value : Value::Text::plain }
                );
            }

            #[test]
            fn text_xml_test(){
                string_to_enum_test(
                    "text/xml",
                    ContentType::Text { value : Value::Text::xml }
                );
            }

            #[test]
            fn video_mpeg_test(){
                string_to_enum_test(
                    "video/mpeg",
                    ContentType::Video { value : Value::Video::mpeg }
                );
            }

            #[test]
            fn video_mp4_test(){
                string_to_enum_test(
                    "video/mp4",
                    ContentType::Video { value : Value::Video::mp4 }
                );
            }

            #[test]
            fn video_quicktime_test(){
                string_to_enum_test(
                    "video/quicktime",
                    ContentType::Video { value : Value::Video::quicktime }
                );
            }

            #[test]
            fn video_x_ms_wmv_test(){
                string_to_enum_test(
                    "video/x-ms-wmv",
                    ContentType::Video { value : Value::Video::x_ms_wmv }
                );
            }

            #[test]
            fn video_x_msvideo_test(){
                string_to_enum_test(
                    "video/x-msvideo",
                    ContentType::Video { value : Value::Video::x_msvideo }
                );
            }

            #[test]
            fn video_x_flv_test(){
                string_to_enum_test(
                    "video/x-flv",
                    ContentType::Video { value : Value::Video::x_flv }
                );
            }

            #[test]
            fn video_x_webm_test(){
                string_to_enum_test(
                    "video/webm",
                    ContentType::Video { value : Value::Video::webm }
                );
            }
        }
    }
    mod http_request_parse_test {
        use crate::HTTPRequest::{ Method, ParserError};

        fn make_HTTP_Request(input_str : &str) -> [u8; 1024]{
            let mut request : [u8; 1024] = [0; 1024];

            input_str.bytes()
                .zip(request.iter_mut())
                .for_each(|(b, ptr)| *ptr = b);

                request
        }

        #[test]
        fn get_parse_test() {
            //test modeled from syntax form https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET
            //GET /index.html

            let get_method_result : Result<Method, ParserError> = Method::new(make_HTTP_Request("GET /index.html HTTP/1.1"));

            assert_eq!(get_method_result.is_ok(), true);
            match get_method_result {
                Ok(get_method_actual) => {
                    match get_method_actual {
                        Method::GET{ file } => {
                            assert_eq!(file, String::from("/index.html"));
                        }
                        _ => {
                            panic!("Incorect variant. Got {} instead", get_method_actual.to_string());
                        }
                    }
                },
                Err(err) => {
                    panic!("{:?}", err);
                }
            }
        }

        //head test
        #[test]
        fn head_parse_test() {
            //test modeled from syntax form https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/HEAD
            //HEAD /index.html

            let get_method_result : Result<Method, ParserError> = Method::new(make_HTTP_Request("HEAD /index.html HTTP/1.1"));

            assert_eq!(get_method_result.is_ok(), true);
            match get_method_result {
                Ok(get_method_actual) => {
                    match get_method_actual {
                        Method::HEAD{ file } => {
                            assert_eq!(file, String::from("/index.html"));
                        }
                        _ => {
                            panic!("Incorect variant. Got {} instead", get_method_actual.to_string());
                        }
                    }
                },
                Err(err) => {
                    panic!("{:?}", err);
                }
            }
        }

        //post test
        #[test]
        fn post_parse_test() {
            //test modeled from syntax form https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST

            let get_method_result : Result<Method, ParserError> = Method::new(
                make_HTTP_Request(
                    "POST /test HTTP/1.1
                    Host: foo.example
                    Content-Type: application/x-www-form-urlencoded
                    Content-Length: 27
                    
                    field1=value1&field2=value2"
                )
            );

            assert_eq!(get_method_result.is_ok(), true);
            match get_method_result {
                Ok(get_method_actual) => {
                    match get_method_actual {
                        Method::POST{ file, body } => {
                            assert_eq!(file, String::from("/test"));

                            assert_eq!(body.content_type.to_string(), "application/x-www-form-urlencoded");

                            assert_eq!(body.content, "field1=value1&field2=value2");
                        }
                        _ => {
                            panic!("Incorect variant. Got {} instead", get_method_actual.to_string());
                        }
                    }
                },
                Err(err) => {
                    panic!("{:?}", err);
                }
            }
        }

        //put test
        #[test]
        fn put_parse_test() {
            //test modeled from syntax form https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/PUT

            let get_method_result : Result<Method, ParserError> = Method::new(
                make_HTTP_Request(
                    "PUT /new.html HTTP/1.1
                    Host: example.com
                    Content-type: text/html
                    Content-length: 16
                    
                    <p>New File</p>"
                )
            );

            assert_eq!(get_method_result.is_ok(), true);
            match get_method_result {
                Ok(get_method_actual) => {
                    match get_method_actual {
                        Method::PUT{ file, body } => {
                            assert_eq!(file, String::from("/new.html"));

                            assert_eq!(body.content_type.to_string(), "text/html");

                            assert_eq!(body.content, "<p>New File</p>");
                        }
                        _ => {
                            panic!("Incorect variant. Got {} instead", get_method_actual.to_string());
                        }
                    }
                },
                Err(err) => {
                    panic!("{:?}", err);
                }
            }
        }

        //delete test
        //connect test
        //options test
        //trace test
        //patch test
    }
}