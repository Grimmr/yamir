use crate::parser::lex::{
    Token
};

#[derive(Debug)]
pub enum MsgType
{
    //F
    MsgLogAttemptToRenderParseErrorWithNoTokenInput,
    LexAttemptedToRunOffEndOfInput,
    ParseConsumedTokenWithoutLookingFirst,

    //E
    LexNoTokenFound,
    ParseWrongTokenInRootParse,
    ParseDatStatementMissingWKeyword,
    ParseDatStatementMissingNum{missing_count:u8},
    ParseNumHasInvalidLexeme
}

#[derive(Debug)]
pub struct Msg
{
    typ: MsgType,
    offset: usize,
    row: usize,
    col: usize
}

#[derive(Debug)]
pub struct MsgLog
{
    msgs:Vec<Msg>
}

impl MsgLog
{
    pub fn new(input:&[u8]) -> MsgLog
    {
        MsgLog {msgs: Vec::new()}
    }

    pub fn add_msg(&mut self, v:Msg)
    {
        self.msgs.push(v);
    }

    pub fn is_empty(&self) -> bool
    {
        self.msgs.len() == 0
    }
}

impl Msg
{
    pub fn new(m:MsgType, offset:usize, row:usize, col:usize) -> Msg
    {
        Msg {typ:m, offset:offset, row:row, col:col}
    }
}

