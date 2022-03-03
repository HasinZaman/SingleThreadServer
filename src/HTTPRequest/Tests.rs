

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
        use crate::HTTPRequest::Method::Method;
        use crate::HTTPRequest::ParserError::ParserError;

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

            let get_method_result : Result<Method, ParserError> = Method::new(
                make_HTTP_Request("GET /index.html HTTP/1.1")
            );

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

            let get_method_result : Result<Method, ParserError> = Method::new(
                make_HTTP_Request("HEAD /index.html HTTP/1.1")
            );

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
                    "POST /test HTTP/1.1\nHost: foo.example\nContent-Type: application/x-www-form-urlencoded\nContent-Length: 27\n\nfield1=value1&field2=value2"
                )
            );
            assert!(get_method_result.is_ok(),"{}", format!("{:?}", get_method_result));

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
                    "PUT /new.html HTTP/1.1\nHost: example.com\nContent-type: text/html\nContent-length: 16\n\n<p>New File</p>"
                )
            );

            assert!(get_method_result.is_ok(),"{}", format!("{:?}", get_method_result));
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