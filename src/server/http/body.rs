use super::request::parser_error::ParserError;
use std::fmt::{Debug, Error, Formatter};

pub use value::Application;
pub use value::Audio;
pub use value::Image;
pub use value::Multipart;
pub use value::Text;
pub use value::Video;

#[derive(Debug)]
pub struct Body {
    pub content_type: ContentType,
    pub content: String,
}

trait VariantName {
    fn get_variant(&self) -> String;
}

//https://www.geeksforgeeks.org/http-headers-content-type/
pub enum ContentType {
    Application(Application),
    Audio(Audio),
    Image(Image),
    Multipart(Multipart),
    Text(Text),
    Video(Video),
}

impl ContentType {
    pub fn new(raw_str: &str) -> Result<ContentType, ParserError> {
        let str_vec: Vec<&str> = raw_str.split("/").collect();

        if str_vec.len() < 2 {
            return Result::Err(ParserError::InvalidMethod(Option::Some(String::from(
                "Start line must have more than 2 elements",
            ))));
        }

        let type_raw: &str = str_vec[0];

        let value: &str = str_vec[1];

        match type_raw {
            "application" => {
                let content_type_value = value::parse_value::<value::Application>(value)?;
                let content_type = ContentType::Application(content_type_value);
                return Ok(content_type);
            }
            "audio" => {
                let content_type_value = value::parse_value::<value::Audio>(value)?;
                let content_type = ContentType::Audio(content_type_value);
                return Ok(content_type);
            }
            "image" => {
                let content_type_value = value::parse_value::<value::Image>(value)?;
                let content_type = ContentType::Image(content_type_value);
                return Ok(content_type);
            }
            "multipart" => {
                let content_type_value = value::parse_value::<value::Multipart>(value)?;
                let content_type = ContentType::Multipart(content_type_value);
                return Ok(content_type);
            }
            "text" => {
                let content_type_value = value::parse_value::<value::Text>(value)?;
                let content_type = ContentType::Text(content_type_value);
                return Ok(content_type);
            }
            "video" => {
                let content_type_value = value::parse_value::<value::Video>(value)?;
                let content_type = ContentType::Video(content_type_value);
                return Ok(content_type);
            }
            _ => {
                return Result::Err(ParserError::InvalidMethod(Option::Some(String::from(
                    "Invalid type",
                ))))
            }
        }
    }
}
impl VariantName for ContentType {
    fn get_variant(&self) -> String {
        match &self {
            ContentType::Application(_value) => String::from("application"),
            ContentType::Audio(_value) => String::from("audio"),
            ContentType::Image(_value) => String::from("image"),
            ContentType::Multipart(_value) => String::from("multipart"),
            ContentType::Text(_value) => String::from("text"),
            ContentType::Video(_value) => String::from("video"),
        }
    }
}
impl ToString for ContentType {
    fn to_string(&self) -> String {
        match &self {
            ContentType::Application(value) => {
                return format!("{}/{}", &self.get_variant(), value.get_variant())
            }
            ContentType::Audio(value) => {
                return format!("{}/{}", &self.get_variant(), value.get_variant())
            }
            ContentType::Image(value) => {
                return format!("{}/{}", &self.get_variant(), value.get_variant())
            }
            ContentType::Multipart(value) => {
                return format!("{}/{}", &self.get_variant(), value.get_variant())
            }
            ContentType::Text(value) => {
                return format!("{}/{}", &self.get_variant(), value.get_variant())
            }
            ContentType::Video(value) => {
                return format!("{}/{}", &self.get_variant(), value.get_variant())
            }
        }
    }
}
impl Debug for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct(&format!("ContentType: {}", self.to_string()))
            .finish()
    }
}

pub mod value {
    use super::super::request::parser_error::ParserError;

    pub trait Constructor<V> {
        fn new(value_raw: &str) -> Result<V, ParserError>;
    }

    #[allow(non_camel_case_types)]
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
        fn get_variant(&self) -> String {
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
            }
        }
    }
    impl Constructor<Application> for Application {
        fn new(value_raw: &str) -> Result<Application, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
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

    #[allow(non_camel_case_types)]
    pub enum Audio {
        mpeg,
        x_ms_wma,
        vnd_rn_realaudio,
        x_wav,
    }
    impl super::VariantName for Audio {
        fn get_variant(&self) -> String {
            match &self {
                Audio::mpeg => return String::from("mpeg"),
                Audio::x_ms_wma => return String::from("x-ms-wma"),
                Audio::vnd_rn_realaudio => return String::from("vnd.rn-realaudio"),
                Audio::x_wav => return String::from("x-wav"),
            }
        }
    }
    impl Constructor<Audio> for Audio {
        fn new(value_raw: &str) -> Result<Audio, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
            match value {
                "mpeg" => return Result::Ok(Audio::mpeg),
                "x-ms-wma" => return Result::Ok(Audio::x_ms_wma),
                "vnd.rn-realaudio" => return Result::Ok(Audio::vnd_rn_realaudio),
                "x-wav" => return Result::Ok(Audio::x_wav),
                _ => panic!("Invalid variant type"),
            }
        }
    }

    #[allow(non_camel_case_types)]
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
        fn get_variant(&self) -> String {
            match &self {
                Image::gif => return String::from("gif"),
                Image::jpeg => return String::from("jpeg"),
                Image::png => return String::from("png"),
                Image::tiff => return String::from("tiff"),
                Image::vnd_microsoft_icon => return String::from("vnd.microsoft.icon"),
                Image::x_icon => return String::from("x-icon"),
                Image::vnd_djvu => return String::from("vnd.djvu"),
                Image::svg_xml => return String::from("svg+xml"),
            }
        }
    }
    impl Constructor<Image> for Image {
        fn new(value_raw: &str) -> Result<Image, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
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

    #[allow(non_camel_case_types)]
    pub enum Multipart {
        mixed,
        alternative,
        related,
        form_data { boundary: String },
    }
    impl super::VariantName for Multipart {
        fn get_variant(&self) -> String {
            match &self {
                Multipart::mixed => String::from("mixed"),
                Multipart::alternative => String::from("alternative"),
                Multipart::related => String::from("related"),
                #[allow(unused_variables)]
                Multipart::form_data { boundary } => String::from("form-data"),
            }
        }
    }
    impl Constructor<Multipart> for Multipart {
        fn new(value_raw: &str) -> Result<Multipart, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
            match value {
                "mixed" => return Result::Ok(Multipart::mixed),
                "alternative" => return Result::Ok(Multipart::alternative),
                "related" => return Result::Ok(Multipart::related),
                "form-data" => {
                    return Result::Ok(Multipart::form_data {
                        boundary: String::from(""),
                    })
                }
                _ => panic!("Invalid variant type"),
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub enum Text {
        css,
        csv,
        html,
        javascript,
        plain,
        xml,
    }
    impl super::VariantName for Text {
        fn get_variant(&self) -> String {
            match &self {
                Text::css => String::from("css"),
                Text::csv => String::from("csv"),
                Text::html => String::from("html"),
                Text::javascript => String::from("javascript"),
                Text::plain => String::from("plain"),
                Text::xml => String::from("xml"),
            }
        }
    }
    impl Constructor<Text> for Text {
        fn new(value_raw: &str) -> Result<Text, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
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

    #[allow(non_camel_case_types)]
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
        fn get_variant(&self) -> String {
            match &self {
                Video::mpeg => String::from("mpeg"),
                Video::mp4 => String::from("mp4"),
                Video::quicktime => String::from("quicktime"),
                Video::x_ms_wmv => String::from("x-ms-wmv"),
                Video::x_msvideo => String::from("x-msvideo"),
                Video::x_flv => String::from("x-flv"),
                Video::webm => String::from("webm"),
            }
        }
    }
    impl Constructor<Video> for Video {
        fn new(value_raw: &str) -> Result<Video, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
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

    pub fn parse_value<V>(value: &str) -> Result<V, ParserError>
    where
        V: Constructor<V>,
    {
        V::new(value)
    }
}
