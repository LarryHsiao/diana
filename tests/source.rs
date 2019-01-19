use diana::source::ConstSource;
use diana::source::Source;

#[test]
fn const_source() {
    let input = &"empty".to_string();
    let source = ConstSource {
        value: input
    };
    let output = source.fetch();
    assert_eq!(
        input,
        output
    )
}
