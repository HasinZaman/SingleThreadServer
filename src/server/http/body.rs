//! body module is responsible for how HTTP body are parsed and stored
use super::request::parser_error::ParserError;
use std::fmt::{Debug, Error, Formatter};

pub use value::Application;
pub use value::Audio;
pub use value::Image;
pub use value::Multipart;
pub use value::Text;
pub use value::Video;

/// Body struct defines the structure of an HTTP body
#[derive(Debug)]
pub struct Body {
    pub content_type: ContentType,
    pub content: Vec<u8>,
}

/// VariantName trait defines method to return string of variant names
trait VariantName {
    /// get_variant returns a string to identify variant
    fn get_variant(&self) -> String;
}

/// ContentType enum define the different types types of [content types](https://www.iana.org/assignments/media-types/media-types.xhtml)
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
    /// Constructor that creates a new ContentType from string
    ///
    /// # Example
    /// ```
    /// let actual: ContentType = ContentType::Application(Application::EDI_X12);
    /// assert_eq!(actual, ContentType::new("application/EDI-X12"));
    /// ```
    pub fn new(raw_str: &str) -> Result<ContentType, ParserError> {
        let str_vec: Vec<&str> = raw_str.split("/").collect();

        if str_vec.len() < 2 {
            return Err(ParserError::InvalidMethod(Some(String::from(
                "Start line must have more than 2 elements",
            ))));
        }

        let type_raw: &str = str_vec[0];

        let value: &str = str_vec[1];

        match type_raw {
            "application" => {
                let content_type_value = value::parse_value::<value::Application>(value)?;
                let content_type = ContentType::Application(content_type_value);
                Ok(content_type)
            }
            "audio" => {
                let content_type_value = value::parse_value::<value::Audio>(value)?;
                let content_type = ContentType::Audio(content_type_value);
                Ok(content_type)
            }
            "image" => {
                let content_type_value = value::parse_value::<value::Image>(value)?;
                let content_type = ContentType::Image(content_type_value);
                Ok(content_type)
            }
            "multipart" => {
                let content_type_value = value::parse_value::<value::Multipart>(value)?;
                let content_type = ContentType::Multipart(content_type_value);
                Ok(content_type)
            }
            "text" => {
                let content_type_value = value::parse_value::<value::Text>(value)?;
                let content_type = ContentType::Text(content_type_value);
                Ok(content_type)
            }
            "video" => {
                let content_type_value = value::parse_value::<value::Video>(value)?;
                let content_type = ContentType::Video(content_type_value);
                Ok(content_type)
            }
            _ => Err(ParserError::InvalidMethod(Some(String::from(
                "Invalid type",
            )))),
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
        format!(
            "{}/{}",
            &self.get_variant(),
            match &self {
                ContentType::Application(value) => {
                    value.get_variant()
                }
                ContentType::Audio(value) => {
                    value.get_variant()
                }
                ContentType::Image(value) => {
                    value.get_variant()
                }
                ContentType::Multipart(value) => {
                    value.get_variant()
                }
                ContentType::Text(value) => {
                    value.get_variant()
                }
                ContentType::Video(value) => {
                    value.get_variant()
                }
            }
        )
    }
}
impl Debug for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct(&format!("ContentType: {}", self.to_string()))
            .finish()
    }
}

pub mod value {
    //! value module defines the specific Enums for each of the ContentType variants
    use super::super::request::parser_error::ParserError;

    /// generic constructor trait define method signature for new methods
    pub trait Constructor<V> {
        fn new(value_raw: &str) -> Result<V, ParserError>;
    }

    /// Application enum defines the variants of ContentType::Application
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
        woff,
    }
    impl super::VariantName for Application {
        fn get_variant(&self) -> String {
            match &self {
                Application::EDI_X12 => String::from("EDI-X12"),
                Application::EDIFACT => String::from("EDIFACT"),
                Application::javascript => String::from("javascript"),
                Application::octet_stream => String::from("octet-stream"),
                Application::ogg => String::from("ogg"),
                Application::pdf => String::from("pdf"),
                Application::xhtml_xml => String::from("xhtml+xml"),
                Application::x_shockwave_flash => String::from("x-shockwave-flash"),
                Application::json => String::from("json"),
                Application::ld_json => String::from("ld+json"),
                Application::xml => String::from("xml"),
                Application::zip => String::from("zip"),
                Application::x_www_form_urlencoded => String::from("x-www-form-urlencoded"),
                Application::woff => String::from("woff"),
            }
        }
    }
    impl Constructor<Application> for Application {
        fn new(value_raw: &str) -> Result<Application, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
            match value {
                "EDI-X12" => Ok(Application::EDI_X12),
                "EDIFACT" => Ok(Application::EDIFACT),
                "javascript" => Ok(Application::javascript),
                "octet-stream" => Ok(Application::octet_stream),
                "ogg" => Ok(Application::ogg),
                "pdf" => Ok(Application::pdf),
                "xhtml+xml" => Ok(Application::xhtml_xml),
                "x-shockwave-flash" => Ok(Application::x_shockwave_flash),
                "json" => Ok(Application::json),
                "ld+json" => Ok(Application::ld_json),
                "xml" => Ok(Application::xml),
                "zip" => Ok(Application::zip),
                "x-www-form-urlencoded" => Ok(Application::x_www_form_urlencoded),
                "woff" => Ok(Application::woff),
                _ => Err(ParserError::InvalidMethod(Some(format!(
                    "{} is not a valid Application variant",
                    value_raw
                )))),
            }
        }
    }

    /// Application enum defines the variants of ContentType::Audio
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
                Audio::mpeg => String::from("mpeg"),
                Audio::x_ms_wma => String::from("x-ms-wma"),
                Audio::vnd_rn_realaudio => String::from("vnd.rn-realaudio"),
                Audio::x_wav => String::from("x-wav"),
            }
        }
    }
    impl Constructor<Audio> for Audio {
        fn new(value_raw: &str) -> Result<Audio, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
            match value {
                "mpeg" => Ok(Audio::mpeg),
                "x-ms-wma" => Ok(Audio::x_ms_wma),
                "vnd.rn-realaudio" => Ok(Audio::vnd_rn_realaudio),
                "x-wav" => Ok(Audio::x_wav),
                _ => Err(ParserError::InvalidMethod(Some(format!(
                    "{} is not a valid Audio variant",
                    value_raw
                )))),
            }
        }
    }

    /// Application enum defines the variants of ContentType::Image
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
                Image::gif => String::from("gif"),
                Image::jpeg => String::from("jpeg"),
                Image::png => String::from("png"),
                Image::tiff => String::from("tiff"),
                Image::vnd_microsoft_icon => String::from("vnd.microsoft.icon"),
                Image::x_icon => String::from("x-icon"),
                Image::vnd_djvu => String::from("vnd.djvu"),
                Image::svg_xml => String::from("svg+xml"),
            }
        }
    }
    impl Constructor<Image> for Image {
        fn new(value_raw: &str) -> Result<Image, ParserError> {
            let value_paramater_vec: Vec<&str> = value_raw.split(";").collect();

            let value: &str = value_paramater_vec[0];
            match value {
                "gif" => Ok(Image::gif),
                "jpeg" => Ok(Image::jpeg),
                "png" => Ok(Image::png),
                "tiff" => Ok(Image::tiff),
                "vnd.microsoft.icon" => Ok(Image::vnd_microsoft_icon),
                "x-icon" => Ok(Image::x_icon),
                "vnd.djvu" => Ok(Image::vnd_djvu),
                "svg+xml" => Ok(Image::svg_xml),
                _ => Err(ParserError::InvalidMethod(Some(format!(
                    "{} is not a valid Image variant",
                    value_raw
                )))),
            }
        }
    }

    /// Application enum defines the variants of ContentType::Multipart
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
                "mixed" => Ok(Multipart::mixed),
                "alternative" => Ok(Multipart::alternative),
                "related" => Ok(Multipart::related),
                "form-data" => Ok(Multipart::form_data {
                    boundary: String::from(""),
                }),
                _ => Err(ParserError::InvalidMethod(Some(format!(
                    "{} is not a valid Multipart variant",
                    value_raw
                )))),
            }
        }
    }

    /// Application enum defines the variants of ContentType::Text
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
                "css" => Ok(Text::css),
                "csv" => Ok(Text::csv),
                "html" => Ok(Text::html),
                "javascript" => Ok(Text::javascript),
                "plain" => Ok(Text::plain),
                "xml" => Ok(Text::xml),
                _ => Err(ParserError::InvalidMethod(Some(format!(
                    "{} is not a valid Text variant",
                    value_raw
                )))),
            }
        }
    }

    /// Application enum defines the variants of ContentType::Video
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
                "mpeg" => return Ok(Video::mpeg),
                "mp4" => return Ok(Video::mp4),
                "quicktime" => return Ok(Video::quicktime),
                "x-ms-wmv" => return Ok(Video::x_ms_wmv),
                "x-msvideo" => return Ok(Video::x_msvideo),
                "x-flv" => return Ok(Video::x_flv),
                "webm" => return Ok(Video::webm),
                _ => panic!("Invalid variant type"),
            }
        }
    }

    /// parse_value is function to create any of the enums in value mod
    ///
    /// # Errors
    /// if the inputted value attribute cannot be parsed into a variant defined by the generic V. The function returns a ParserError.
    pub fn parse_value<V>(value: &str) -> Result<V, ParserError>
    where
        V: Constructor<V>,
    {
        V::new(value)
    }
}
