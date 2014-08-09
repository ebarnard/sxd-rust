extern crate xpath;

use xpath::token;
use xpath::token::XPathToken;
use xpath::tokenizer::TokenResult;

struct XPathTokenDisambiguator<I> {
    source: I,
}

impl<I> XPathTokenDisambiguator<I> {
    fn new(source: I) -> XPathTokenDisambiguator<I> {
        XPathTokenDisambiguator{
            source: source,
        }
    }
}

impl<I> Iterator<TokenResult> for XPathTokenDisambiguator<I> {
    fn next(&mut self) -> Option<TokenResult> {
        None
    }
}

fn all_tokens_raw<I: Iterator<TokenResult>>(tokenizer: I) -> Result<Vec<XPathToken>, & 'static str> {
    std::result::collect(tokenizer)
}

fn all_tokens<I: Iterator<TokenResult>>(tokenizer: I) -> Vec<XPathToken> {
    match all_tokens_raw(tokenizer) {
        Ok(toks) => toks,
        Err(msg) => fail!(msg),
    }
}

#[test]
fn disambiguates_node_test_functions() {
    // Would prefer parametric tests
    for name in ["comment", "text", "processing-instruction", "node"].iter() {
        let input_tokens: Vec<TokenResult> = vec!(
            Ok(token::String(name.to_string())),
            Ok(token::LeftParen),
        );

        let disambig = XPathTokenDisambiguator::new(input_tokens.move_iter());

        assert_eq!(all_tokens(disambig),
                   vec!(token::NodeTest(name.to_string()),
                        token::LeftParen));
    }
}

#[test]
fn name_followed_by_left_paren_becomes_function_name() {
    let input_tokens: Vec<TokenResult> = vec!(
        Ok(token::String("test".to_string())),
        Ok(token::LeftParen),
     );

  let disambig = XPathTokenDisambiguator::new(input_tokens.move_iter());

  assert_eq!(all_tokens(disambig),
             vec!(token::Function("test".to_string()),
                  token::LeftParen));
}

#[test]
fn name_followed_by_double_colon_becomes_axis_name() {
    let input_tokens: Vec<TokenResult> = vec!(
        Ok(token::String("test".to_string())),
        Ok(token::DoubleColon),
    );

  let disambig = XPathTokenDisambiguator::new(input_tokens.move_iter());

    assert_eq!(all_tokens(disambig),
             vec!(token::Axis("test".to_string()),
                  token::DoubleColon));
}
