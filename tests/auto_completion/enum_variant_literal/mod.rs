mod test {
    use teo_parser::{auto_complete_items, parse};

    #[test]
    fn completion_items_for_enum_variant() {
        let path_buf = std::env::current_dir().unwrap().join("tests/auto_completion/enum_variant_literal/schemas/01.teo");
        let path = path_buf.to_str().unwrap();
        let (schema, _) = parse(path, None, None);
        let completions = auto_complete_items(&schema, path, (22, 14));
        assert_eq!(completions.len(), 2);
    }

    #[test]
    fn completion_items_for_through_fields() {
        let path_buf = std::env::current_dir().unwrap().join("tests/auto_completion/enum_variant_literal/schemas/02.teo");
        let path = path_buf.to_str().unwrap();
        let (schema, _) = parse(path, None, None);
        let completions = auto_complete_items(&schema, path, (53, 40));
        assert_eq!(completions.len(), 2);
        assert_eq!(completions.first().unwrap().label.as_str(), "artist");
        assert_eq!(completions.last().unwrap().label.as_str(), "song");
    }

    #[test]
    fn completion_items_for_last_argument_synthesized_enum_variant_reference() {
        let path_buf = std::env::current_dir().unwrap().join("tests/auto_completion/enum_variant_literal/schemas/04.teo");
        let path = path_buf.to_str().unwrap();
        let (schema, _) = parse(path, None, None);
        let completions = auto_complete_items(&schema, path, (27, 46));
        assert_eq!(completions.len(), 4);
        assert_eq!(completions.iter().find(|c| c.label.as_str() == "id").is_some(), true);
        assert_eq!(completions.iter().find(|c| c.label.as_str() == "name").is_some(), true);
        assert_eq!(completions.iter().find(|c| c.label.as_str() == "userId").is_some(), true);
        assert_eq!(completions.iter().find(|c| c.label.as_str() == "userIs").is_some(), true);
    }

    #[test]
    fn completion_items_for_data_set_record_inside_array() {

    }

    #[test]
    fn completion_items_for_data_set_record() {

    }
}