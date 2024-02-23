use lex::{
    Token
    TokenType
};

use crate::util::error_log::{
    Msg,
    MsgLog,
    MsgType
};


enum Node
{
    Root{declerations: Node},
    Data{width: Node, value:Node},
    Num{value: u32}
}

fn parse_from_bytes(input:&[u8]) -> (Result<Node>, MsgLog) 
{
    //create the msg log we will be using
    let msgs = MsgLog::new();
    
    //stage one lex
    let (tokens, lex_msgs) = lex::lex(input);

    //lex errors are show stopers 
    if !lex_msgs.is_empty()
    {
        return (None, lex_msgs);
    }

    //stage 2 parse a root node and return
    parse_root(tokens, msgs)
}

fn parse_root(tokens: Vec<Token>, log:&mut MsgLog)
{
    match
}