use convert_case::Case;
use proptest::{
    prelude::{Just, Strategy},
    prop_compose, prop_oneof,
    sample::select,
};

prop_compose! {
    pub fn whitespace_variations()(
        leading in select(vec!["", " ", "  ", "\t", " \t", "\r", "\n"]).prop_map(|x| x.to_string()),
        trailing in select(vec!["", " ", "  ", "\t", " \t", "\r", "\n"]).prop_map(|x| x.to_string()),
        internal in select(vec![";", "; ", " ; ", " ;", ";\t", "\t;\t"]).prop_map(|x| x.to_string()),
    ) -> (String, String, String) {(leading, trailing, internal)}
}

prop_compose! {
    pub fn case_strategy()(
        s in prop_oneof![
            Just(Case::Lower),
            Just(Case::Upper),
            Just(Case::Title),
    ]) -> Case<'static> { s }
}

pub fn case_convertion(mime_type: &str, case: Case) -> String {
    match case {
        Case::Lower => mime_type.to_lowercase(),
        Case::Upper => mime_type.to_uppercase(),
        Case::Title => {
            if let Some((main_type, sub_type)) = mime_type.split_once('/') {
                format!(
                    "{}/{}",
                    title_case_word(main_type),
                    title_case_preserving_structure(sub_type)
                )
            } else {
                title_case_word(mime_type)
            }
        }
        _ => mime_type.to_string(),
    }
}

fn title_case_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first
            .to_uppercase()
            .chain(chars.as_str().to_lowercase().chars())
            .collect(),
    }
}

fn title_case_preserving_structure(subtype: &str) -> String {
    let mut result = String::new();
    let mut word_start = true;

    for ch in subtype.chars() {
        match ch {
            '.' | '-' | '+' => {
                result.push(ch);
                word_start = true;
            }
            c if word_start => {
                result.push(c.to_uppercase().next().unwrap_or(c));
                word_start = false;
            }
            c => {
                result.push(c.to_lowercase().next().unwrap_or(c));
            }
        }
    }

    result
}
