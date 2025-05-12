// Parameters
// syntax
// a-block-with-two-parameters [pos param 1] [pos param 2]
// a-block-with-keyword-parameter [:delim ,] [pos param 1]
// a-block-with-keyword-boolean-parameter [:use-graphics t]

use std::collections::BTreeMap;

/// Input from one line
/// body: the whole input after the template name
/// positional: each portion of input in order
/// named: keyword input
/// text_body: the whole input excluding any named parameters
///
/// TODO: examples
/// TODO: support for validation file
/// e.g. text.validation.toml
/// args=1
/// text=1
/// named=[]
/// verse.validation.toml
/// args=2
/// text=1
/// named=[ref]
pub(in crate::process) struct LineInput {
    body: String,
    positional: Vec<String>,
    named: BTreeMap<String, String>,
    text_body: Vec<String>,
}

// TODO: function to read line

pub(in crate::process) fn parse_line(line: String) -> LineInput {
    // TODO
    LineInput {
        body: line.clone(),
        positional: vec![line.clone()],
        named: BTreeMap::new(),
        text_body: vec![line],
    }
}

#[test]
fn split_assert() {
    let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
    assert_eq!(v, ["Mary", "had", "a little lambda"]);
}
