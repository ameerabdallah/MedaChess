use std::{str::SplitWhitespace, io};

static START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

struct Uci {
    position: Position,
    stateStack: Vec<StateInfo>,
}

impl Uci {
    fn uci_loop() {
        // let mut pos = Position::from_fen(START_FEN).unwrap();
        let mut line = String::new();
        let mut stdin = io::stdin();
        loop {
            line.clear();
            stdin.read_line(&mut line).expect(ErrFatal::READ_IO);
            
            let mut args = line.split_whitespace();
            match args.next() {
                Some("quit") | Some("stop") => break,
                Some("setoption") => Self::setoption(args),
                Some("uci") => {
                    println!("id name {}\nid author {}\n{}\nuciok", "MedaChess", "Ameer Abdallah", "");
                },
                Some("isready") => println!("readyok"),
                Some("setoption") => Self::setoption(args),
                Some("position") => Self::position(args),
                Some("ucinewgame") => ,
                Some("go") => Some(DownstreamCommand::Go),
                _ => None,
            };
        }
    }

    fn uci() {

    }

    fn debug(args: SplitWhitespace) {
        let mut arg = args.next();
        match arg {
            Some("on") => Some(DownstreamCommand::Debug(true)),
            Some("off") => Some(DownstreamCommand::Debug(false)),
        }
    }
    
    fn setoption(args: SplitWhitespace) {
        args.next(); // skip "name" token
    
        let name = String::from("");
        let value = String::from("");
    
        loop {
            let token = args.next();
            match token {
                Some("value") => break,
                Some(token) => name = format!("{} {}", name, token),
                None => break,
            }
        }
    
        loop {
            let token = args.next();
            if token == None {
                break;
            }
            value = format!("{} {}", value, token.unwrap());
        }
        
    }
    
    fn position(args: SplitWhitespace) -> Option<DownstreamCommand> {
    
    }
}