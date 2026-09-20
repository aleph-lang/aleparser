#[test]
fn typed_function_round_trips_through_parse_and_generate() {
    let source = "fun square(n: Int) -> Int | pure = {\n    n * n\n}";
    let tree = aleparser::parse(source.to_string());
    let regenerated = alegen::generate(tree);
    assert_eq!(regenerated, source);
}

#[test]
fn sum_type_declaration_round_trips_through_parse_and_generate() {
    let source = "type Shape = Circle(Float) | Rect(Float, Float)";
    let tree = aleparser::parse(source.to_string());
    let regenerated = alegen::generate(tree);
    assert_eq!(regenerated, source);
}

#[test]
fn typed_function_without_effects_round_trips_through_parse_and_generate() {
    let source = "fun square(n: Int) -> Int = {\n    n * n\n}";
    let tree = aleparser::parse(source.to_string());
    let regenerated = alegen::generate(tree);
    assert_eq!(regenerated, source);
}

#[test]
fn untyped_function_still_round_trips_unchanged() {
    let source = "fun square(n) = {\n    n * n\n}";
    let tree = aleparser::parse(source.to_string());
    let regenerated = alegen::generate(tree);
    assert_eq!(regenerated, source);
}
