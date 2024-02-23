use crate::util::error_log::{
    Msg,
    MsgLog,
    MsgType
};

#[derive(Debug)]
enum TokenType
{
    Dat,
    W,
    Num,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Hash,
    Ident,
    Coma,
    Colon,
    Bang,
    At,
    Dollar,
    Amp,
    Add
}

#[derive(Debug)]
struct Token<'a>
{
    typ:TokenType,
    lexeme:& 'a [u8],
    offset:usize,
    row:usize,
    col:usize
}

#[derive(Debug)]
struct LexMachine<'a>
{
    to_lex:& 'a [u8],
    tokens:Vec<Token<'a>>,
    offset:usize,
    row:usize,
    col:usize,
    msgs:MsgLog
}

impl<'a> LexMachine<'a>
{
    fn new(input:& 'a [u8]) -> LexMachine<'a>
    {
        let mut machine = LexMachine {to_lex: input, tokens: Vec::new(), offset: 0, row: 0, col: 0, msgs: MsgLog::new()};
        machine.handle_next_whitespace();
        machine
    }

    fn lex_next_token(& 'a mut self) -> &mut LexMachine
    {
        if self.to_lex.len() == 0
        {
            self.msgs.addMsg(Msg::new_from_lex_machine(MsgType::LexAttemptedToRunOffEndOfInput, self));
            return self;
        }

        //single char cases
        match self.to_lex[0]
        {
            b'(' => { self.handle_token_creation(TokenType::LParen, 1); return self; },
            b')' => { self.handle_token_creation(TokenType::RParen, 1); return self; },
            b'{' => { self.handle_token_creation(TokenType::LBrace, 1); return self; },
            b'}' => { self.handle_token_creation(TokenType::RBrace, 1); return self; },
            b'#' => { self.handle_token_creation(TokenType::Hash, 1); return self; },
            b',' => { self.handle_token_creation(TokenType::Coma, 1); return self; },
            b':' => { self.handle_token_creation(TokenType::Colon, 1); return self; },
            b'!' => { self.handle_token_creation(TokenType::Bang, 1); return self; },
            b'@' => { self.handle_token_creation(TokenType::At, 1); return self; },
            b'$' => { self.handle_token_creation(TokenType::Dollar, 1); return self; },
            b'&' => { self.handle_token_creation(TokenType::Amp, 1); return self; },
            _ => ()
        }

        //buffer ident or keyword case
        if Self::is_valid_ident_char(self.to_lex[0], true)
        {
            //find first non-ident character 
            let mut word_end = 1;
            while Self::is_valid_ident_char(self.to_lex[word_end], false)
            {
                word_end += 1;
            }

            //create lexeme buffer
            let buffer = &self.to_lex[0..word_end];

            //create token and return a reference
            match buffer
            {
                b"dat" => self.handle_token_creation(TokenType::Dat, word_end),
                b"ADD" => self.handle_token_creation(TokenType::Add, word_end),
                _ => self.handle_token_creation(TokenType::Ident, word_end)
            }

            return self;
        }

        //buffer number case
        if self.to_lex[0].is_ascii_digit()
        {
            //find first non-digit character 
            let mut word_end = 1;
            while self.to_lex[0].is_ascii_digit()
            {
                word_end += 1;
            }

            //create token
            self.handle_token_creation(TokenType::Num, word_end);

            return self;
        }

        self.msgs.addMsg(Msg::new_from_lex_machine(MsgType::LexNoTokenFound, self));
        return self;
    }

    fn handle_token_creation(&mut self, t:TokenType, consume:usize)
    {
        //update the state to the post token lex state for our token
        self.tokens.push(Token{typ:t, lexeme:&self.to_lex[0..consume], offset: self.offset, row:self.row, col:self.col});
        self.to_lex = &self.to_lex[consume..];
        self.offset += consume;
        self.col += consume;

        //eat trailing whitespace
        self.handle_next_whitespace();

    }

    fn handle_next_whitespace(&mut self)
    {
        if self.to_lex.len() == 0
        {
            return;
        }
        
        while self.to_lex[0] == b' ' || self.to_lex[0] == b'\n'
        {
            self.offset += 1;
            self.col += 1;
            self.to_lex = &self.to_lex[1..];
            if self.to_lex[0] == b'\n'
            {
                self.col = 0;
                self.row += 1;
            }
        }
    }

    fn is_valid_ident_char(c:u8, first:bool) -> bool
    {
        c == b'_' || c == b'-' || c.is_ascii_alphabetic() || (!first && c.is_ascii_digit()) 
    }
}

impl Msg
{
    fn new_from_lex_machine(t: MsgType, machine: &LexMachine) -> Msg
    {
        Msg::new(t, machine.offset, machine.row, machine.col)
    }
}

pub fn test()
{
    let mut m = &mut LexMachine::new(b"]");
    m = m.lex_next_token();
    println!("{:?}", m);
}