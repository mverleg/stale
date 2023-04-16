//TODO @mark: delete either this if lalrpop turns out better, or delete lalrpop if going this way

use ::std::path::PathBuf;
use ::std::sync::LazyLock;

use ::regex::Regex;
use itertools::Itertools;
use steel_api::log::{debug, trace};

use crate::ast::AST;
use crate::ast::OpCode;
use crate::parser::custom::Token::OpSymbol;
use crate::SteelErr;

pub fn parse_str(src_pth: PathBuf, code: &str) -> Result<AST, SteelErr> {
    let tokens = tokenize(src_pth, code)?;
    unimplemented!()  //TODO @mark:
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ParenthesisOpen,
    ParenthesisClose,
    OpSymbol(OpCode),
    Number(f64),
}

trait Tokenizer {
    fn regex(&self) -> &Regex;

    fn token_for(&self, cap_group: Option<&str>) -> Token;
}

struct FixedTokenTokenizer(Regex, Token);

impl Tokenizer for FixedTokenTokenizer {
    fn regex(&self) -> &Regex {
        &self.0
    }

    fn token_for(&self, ignored: Option<&str>) -> Token {
        debug_assert!(ignored.is_none(), "no capture group expected for this tokenizer");
        self.1.clone()
    }
}

struct OpSymbolTokenizer(Regex);

impl Tokenizer for OpSymbolTokenizer {
    fn regex(&self) -> &Regex {
        &self.0
    }

    fn token_for(&self, op_sym: Option<&str>) -> Token {
        Token::OpSymbol(match op_sym {
            "+" => OpCode::Add,
            "-" => OpCode::Sub,
            "*" => OpCode::Mul,
            "/" => OpCode::Div,
            _ => unreachable!(),
        })
    }
}

struct NumberTokenizer(Regex);

impl Tokenizer for NumberTokenizer {
    fn regex(&self) -> &Regex {
        &self.0
    }

    fn token_for(&self, num_repr: Option<&str>) -> Token {
        match num_repr.parse() {
            Ok(num) => Token::Number(num),
            Err(_err) => unimplemented!(),  //TODO @mark: error handling (e.g. too large nr? most invalid input is handled by regex)
        }
    }
}

//TODO @mark: pity about dyn, see if it gets optimized
static TOKENIZERS: LazyLock<[dyn Tokenizer; 4]> = LazyLock::new(|| {
    debug!("start creating tokenizers (compiling regexes)");
    let tokenizers = [
        FixedTokenTokenizer(Regex::new(r"^\s*\(\s*").unwrap(), Token::ParenthesisOpen),
        FixedTokenTokenizer(Regex::new(r"^\s*\)[ \t]*").unwrap(), Token::ParenthesisClose),
        OpSymbolTokenizer(Regex::new(r"^\s*([*+\-/])\s*").unwrap()),
        NumberTokenizer(Regex::new(r"^\s*\(\s*").unwrap()),
    ];
    debug!("finished creating tokenizers (compiling regexes)");
    tokenizers
});

pub fn tokenize(src_pth: PathBuf, full_code: &str) -> Result<Vec<Token>, SteelErr> {
    let mut tokens = Vec::new();
    let mut ix = 0;
    while ix < full_code.len() {
        //TODO @mark: drop '...\n' continuations
        let code = &full_code[ix..];
        eprintln!("ix={ix} code='{}'", code.chars().take(40).join(""));  //TODO @mark: TEMPORARY! REMOVE THIS!
        for tokenizer in TOKENIZERS {
            if let Some(caps) = tokenizer.regex().captures_iter(code).next() {
                let mtch = caps.get(0).unwrap().as_str();
                let grp = caps.get(0).map(|g| g.as_str());
                let token = tokenizer.token_for(grp);
                eprintln!("match {token:?} in '{cap}' from {ix} to {}", ix + grp.len());
                //TODO @mark: change to trace ^
                tokens.push(token);
                ix += grp.len();
                debug_assert!(grp.len() > 0);
                continue;
            }
        }
        unreachable!("unexpected end of input at #{ix} ('{}')", code.chars().next().unwrap())
    }
    Ok(tokens)
}

#[cfg(test)]
mod tokens {
    use super::*;

    #[test]
    fn allow_whitespace_after_open_parenthesis() {
        let tokens = tokenize(PathBuf::from("test"), "(\n)");
        assert_eq!(tokens, Ok(vec![Token::ParenthesisOpen, Token::ParenthesisClose]));
    }

    #[test]
    fn handle_non_ascii_strings() {
        let tokens = tokenize(PathBuf::from("test"), "\"你好\"");
        assert_eq!(tokens, Ok(vec![Token::ParenthesisOpen, Token::ParenthesisClose]));
    }

    #[test]
    fn simple_arithmetic() {
        let tokens = tokenize(PathBuf::from("test"), "(3) + (4 / 2)");
        assert_eq!(tokens, Ok(vec![Token::ParenthesisOpen, Token::Number(3.), Token::ParenthesisClose, Token::OpSymbol(OpCode::Add),
                Token::ParenthesisOpen, Token::Number(4.), Token::OpSymbol(OpCode::Div), Token::Number(2.), Token::ParenthesisClose]));
    }
}
