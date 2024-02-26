use super::lex::{
    Token,
    TokenType
};

use crate::util::error_log::{
    Msg,
    MsgLog,
    MsgType
};

#[derive(Debug, PartialEq)]
pub enum Node
{
    Root{statements: Vec<Node>},
    Data{width: Box<Node>, value:Box<Node>},
    Num{value: u32},
}

struct ParseMachine<'a>
{
    msgs: MsgLog,
    tokens: & 'a [Token<'a>],
}

impl ParseMachine<'_>
{
    fn new<'a>(input:& 'a [Token<'a>], msgs:MsgLog) -> ParseMachine<'a>
    {
        let mut machine = ParseMachine {msgs: msgs, tokens:input};
        machine.tokens = &machine.tokens[..];
        machine
    }

    fn parse_root(&mut self) -> Option<Node>
    {
        let mut children = Vec::new();
        
        while self.tokens.len() > 0
        {
            let child;
            match self.peek_type()
            {
                TokenType::Dat => child = self.parse_data(),
                //TokenType::Hash => println!("func"),
                //TokenType::Colon => println!("label"),
                _ => {self.msgs.add_msg(Msg::new_from_parse_machine(MsgType::ParseWrongTokenInRootParse, self)); return None;}
            }
            match child
            {
                Some(c) => children.push(c),
                None => return None
            }
        }

        return Some(Node::Root {statements: children});
    }

    fn parse_data(&mut self) -> Option<Node>
    {
        //DAT
        self.consume_token(TokenType::Dat);
        //W
        if self.peek_type() != &TokenType::W
        {
            self.msgs.add_msg(Msg::new_from_parse_machine(MsgType::ParseDatStatementMissingWKeyword, self));
            return None;
        }
        self.consume_token(TokenType::W);
        //NUM
        if self.peek_type() != &TokenType::Num
        {
            self.msgs.add_msg(Msg::new_from_parse_machine(MsgType::ParseDatStatementMissingNum {missing_count: 2}, self));
            return None;
        }
        let width;
        match self.parse_num()
        {
            Some(w) => width = w,
            None => return None 
        }
        //NUM
        if self.peek_type() != &TokenType::Num
        {
            self.msgs.add_msg(Msg::new_from_parse_machine(MsgType::ParseDatStatementMissingNum {missing_count: 1}, self));
            return None;
        }
        let value;
        match self.parse_num()
        {
            Some(v) => value = v,
            None => return None 
        }

        Some(Node::Data {width: Box::new(width), value: Box::new(value)})
    }

    fn parse_num(&mut self) -> Option<Node>
    {
        let num_tok = self.consume_token(TokenType::Num);
        let found_val = std::str::from_utf8(num_tok.lexeme).expect("we assume this will work if it doesn't things are very bad").parse();
        match found_val
        {
            Ok(v) => return Some(Node::Num {value: v}),
            Err(_) => { self.msgs.add_msg(Msg::new_from_parse_machine(MsgType::ParseNumHasInvalidLexeme, self)); return None; } 
        }
    }

    fn consume_token(&mut self, exp:TokenType) -> &Token
    {
        if self.tokens[0].typ != exp
        {
            self.msgs.add_msg(Msg::new_from_parse_machine(MsgType::ParseConsumedTokenWithoutLookingFirst, self));
        }
        
        let t = &self.tokens[0];
        self.tokens = &self.tokens[1..];
        t
    }

    fn peek_type(&self) -> &TokenType
    {
        &self.tokens[0].typ
    }
}

impl Msg
{
    fn new_from_parse_machine(t: MsgType, machine: &ParseMachine) -> Msg
    {
        Msg::new(t, machine.tokens[0].offset, machine.tokens[0].row, machine.tokens[0].col)
    }
}

fn parse(tokens:&[Token], msgs:MsgLog) -> (Option<Node>, MsgLog) 
{
    let mut machine = ParseMachine::new(tokens, msgs);
    let root = machine.parse_root();
    return (root, machine.msgs);
} 

pub fn parse_from_bytes(input:&[u8]) -> (Option<Node>, MsgLog, Vec<Token>) 
{
    //lex the bytes to tokens
    let (tokens, msgs) = super::lex::lex(input);

    //parse
    let (root, msgs) = parse(&tokens, msgs);

    //return result
    (root, msgs, tokens)
}

