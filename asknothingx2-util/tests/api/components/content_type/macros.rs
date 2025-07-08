macro_rules! define_tests {
    ($type:ident) => {
        use std::str::FromStr;

        use asknothingx2_util::api::content_type::ContentType;
        use http::HeaderValue;
        use proptest::prelude::*;
        use proptest::{prop_compose, prop_oneof};

        use crate::api::components::content_type::{helper::*, parameter_strategy, params::Params};

        proptest::prop_compose! {
            /// input, mime_type
            fn base_with_case()(
                case in case_strategy(),
                mt in base_stragety()
            ) -> (String, $type) {
                (
                    case_convertion(mt.as_str(), case),
                    mt,
                )
            }
        }

        proptest::prop_compose! {
            /// input, mime_type, params_str, params_vec
            fn with_param()(
                inp in base_with_case(),
                params in parameter_strategy()
            ) -> (String, $type, String, Vec<Params>) {
                let (inp, mt) = inp;
                let (par_str, exp_par) = params;
                if par_str.is_empty(){
                    (inp, mt, par_str, exp_par)
                } else {
                    let input = format!("{}; {}", inp, par_str);
                    (input, mt, par_str, exp_par)
                }
            }
        }

        proptest::prop_compose! {
            pub fn with_whitespace()(
                inp in base_with_case(),
                params in parameter_strategy(),
                ws in whitespace_variations()
           ) -> (String, String, $type, String, Vec<Params>) {
                let (inp, mt) = inp;
                let (par_str, exp_par) = params;
                let (lead, trail, sep) = ws;

                let full_input = if par_str.is_empty() {
                    inp.clone()
                } else {
                    format!("{}; {}", inp, par_str)
                };

                let ws_inp = format!("{}{}{}", lead, full_input, trail);
                let final_input = if full_input.contains(';') {
                    ws_inp.replace(";", &sep)
                } else {
                    ws_inp
                };

                (final_input, inp, mt, par_str, exp_par)
            }
        }

        proptest::proptest! {
            #[test]
            fn roundtrip(exp_mt in base_stragety()) {
                let mime_str = exp_mt.as_str();
                let parsed = mime_str.parse::<$type>().unwrap();
                prop_assert_eq!(mime_str, parsed);

                let ct = ContentType::from_str(mime_str).unwrap();
                prop_assert_eq!(ct, ContentType::$type(exp_mt));
            }

            #[test]
            fn header_value_conversion(exp_mt in base_stragety()) {
                // Test HeaderValue roundtrip
                let header_value = exp_mt.as_header_value();
                let parsed = $type::from_header_value(&header_value).unwrap();
                prop_assert_eq!(parsed, exp_mt);

                // Test to_header_value
                let header_value2 = exp_mt.to_header_value();
                prop_assert_eq!(header_value.to_str().unwrap(), header_value2.to_str().unwrap());
            }

            #[test]
            fn display_trait_consistency(app in base_stragety()) {
                // Display should match as_str
                prop_assert_eq!(format!("{}", app), app.as_str());
                prop_assert_eq!(app.to_string(), app.as_str());
            }

            #[test]
            fn partial_eq_implementations(app in base_stragety()) {
                let mime_str = app.as_str();
                let mime_string = String::from(mime_str);

                // Test various PartialEq implementations
                prop_assert_eq!(mime_str, app);
                prop_assert_eq!(mime_string, app);

                // Test with ContentType
                let ct = ContentType::$type(app);
                prop_assert_eq!(app, ct.clone());
                prop_assert_eq!(ct, app);
            }

            #[test]
            fn try_from_implementations(app in base_stragety()) {
                let mime_str = app.as_str();
                let mime_string = String::from(mime_str);

                // Test TryFrom<&str>
                let parsed1 = $type::try_from(mime_str).unwrap();
                prop_assert_eq!(parsed1, app);

                // Test TryFrom<String>
                let parsed2 = $type::try_from(mime_string).unwrap();
                prop_assert_eq!(parsed2, app);

                // Test TryFrom<&HeaderValue>
                let header_value = HeaderValue::from_str(mime_str).unwrap();
                let parsed3 = $type::try_from(&header_value).unwrap();
                prop_assert_eq!(parsed3, app);
            }

            #[test]
            fn from_implementations(app in base_stragety()) {
                // Test From<Application> for String
                let string_result: String = app.into();
                prop_assert_eq!(string_result, app.as_str());

                // Test From<Application> for HeaderValue
                let header_result: HeaderValue = app.into();
                prop_assert_eq!(header_result.to_str().unwrap(), app.as_str());
            }

            #[test]
            fn parsing_with_parameters((input, mime_type, _pars_str, _exp_par ) in with_param()) {
                let parsed = input.parse::<$type>().unwrap();
                prop_assert_eq!(parsed, mime_type);

                let ct_parsed = ContentType::from_str(&input).unwrap();
                prop_assert_eq!(ct_parsed, ContentType::$type(mime_type));
            }

            #[test]
            fn case_insensitive_parsing((input, _mt) in base_with_case()) {
                let parsed_mixed = input.parse::<$type>().unwrap();

                // prop_assert_eq!(parsed_mixed, input);
                prop_assert_eq!(input, parsed_mixed);
            }
        }
    };
}
