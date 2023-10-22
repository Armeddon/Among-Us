use std::io;
use rand::Rng;

pub enum Color {
    Red,
    Blue,
    Purple,
    Green,
    Yellow,
    Cyan,
    Black,
    White,
    Brown,
    Lime,
    Pink,
    Orange,
}

pub enum Command {
    Sus(Color),
    Vented,
    Sussy,
    Electrical,
    Who,
    Where
}

pub struct AmongUs {
    accumulator1: u8,
    accumulator2: u8,
    stack: Vec<u8>,
}

impl Clone for Color {
    fn clone(&self) -> Color{
        match self {
            Color::Red => Color::Red,
            Color::Blue => Color::Blue,
            Color::Purple => Color::Purple,
            Color::Green => Color::Green,
            Color::Yellow => Color::Yellow,
            Color::Cyan => Color::Cyan,
            Color::Black => Color::Black,
            Color::White => Color::White,
            Color::Brown => Color::Brown,
            Color::Lime => Color::Lime,
            Color::Pink => Color::Pink,
            Color::Orange => Color::Orange,
        }
    }
}
impl Copy for Color {}

impl AmongUs {
    pub fn new() -> AmongUs {
        AmongUs {
            accumulator1: 0,
            accumulator2: 0,
            stack: Vec::new(),
        }
    }
    pub fn exec(&mut self, cmd: Command) -> Option<bool> {
        match cmd {
            Command::Vented => {
                self.accumulator2 += 10;
            }
            Command::Sussy => {
                self.accumulator2 -= 1;
            }
            Command::Electrical => {
                self.accumulator2 = 0;
            }
            Command::Sus(clr) => {
                match clr {
                    Color::Red => {
                        self.accumulator1 += 1;
                    }
                    Color::Blue => {
                        self.stack.push(self.accumulator1);
                    }
                    Color::Purple => {
                        self.stack.pop();
                    }
                    Color::Green => {
                        if let Some(output) = self.stack.last() {
                            let output = char::from_u32(*output as u32).unwrap();
                            println!("{}", output);
                        }
                    }
                    Color::Yellow => {
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        let input: char = input.trim().parse().unwrap();
                        self.stack.push(input as u8);
                    }
                    Color::Cyan => {
                        let rnum = rand::thread_rng().gen_range(0..self.accumulator1);
                        for _ in 0..rnum {
                            self.stack.pop();
                        }
                    }
                    Color::Black => {
                        if let Some(output) = self.stack.last() {
                            println!("{}", output);
                        }
                    }
                    Color::White => {
                        self.accumulator1 -= 1;
                    }
                    Color::Brown => {
                        if let Some(value) = self.stack.last() {
                            self.accumulator1 = *value;
                        }
                    }
                    Color::Lime => {
                        if let Some(last) = self.stack.pop() {
                            self.stack.push(last*2);
                        }
                    }
                    Color::Pink => {
                        self.accumulator1 = 0;
                    }
                    Color::Orange => {
                        self.accumulator1 += 10;
                    }
                }
            }
            Command::Who => {
                todo!();
            }
            Command::Where => {
                if let Some(value) = self.stack.last() {
                    return Option::Some(*value == self.accumulator2);
                }
            }
        }
        Option::None
    }
}

#[macro_export]
macro_rules! interpret {
    {mut $amongus: expr, $($line: stmt)*} => {
        {
            let mut clr = Color::Black;
            let mut v:Vec<&str> = Vec::new();
            let mut save_cmds = false;
            $(
                let mut ln = stringify!($line);
                if save_cmds{
                    v.push(ln);
                }

                loop {
                    if let Some(lst) = v.last() {
                        if !save_cmds {
                            ln = *lst;
                        }
                    }
                    match ln {
                        "SUS;" => {
                            let cmd = Command::Sus(clr);
                            $amongus.exec(cmd);
                        }
                        "RED;" => {
                            clr = Color::Red;
                        }
                        "BLUE;" => {
                            clr = Color::Blue;
                        }
                        "PURPLE;" => {
                            clr = Color::Purple;
                        }
                        "GREEN;" => {
                            clr = Color::Green;
                        }
                        "YELLOW;" => {
                            clr = Color::Yellow;
                        }
                        "CYAN;" => {
                            clr = Color::Cyan;
                        }
                        "BLACK;" => {
                            clr = Color::Black;
                        }
                        "WHITE;" => {
                            clr = Color::White;
                        }
                        "BROWN;" => {
                            clr = Color::Brown;
                        }
                        "LIME;" => {
                            clr = Color::Lime;
                        }
                        "PINK;" => {
                            clr = Color::Pink;
                        }
                        "ORANGE;" => {
                            clr = Color::Orange;
                        }
                        "VENTED;" => {
                            let cmd = Command::Vented;
                            $amongus.exec(cmd);
                        }
                        "SUSSY;" => {
                            let cmd = Command::Sussy;
                            $amongus.exec(cmd);
                        }
                        "ELECTRICAL;" => {
                            let cmd = Command::Electrical;
                            $amongus.exec(cmd);
                        }
                        "WHO;" | "WHO?;" => {
                            v = Vec::new();
                            save_cmds = true;
                            v.push(&ln);
                        }
                        "WHERE;" | "WHERE?;" => {
                            let cmd = Command::Where;
                            if let Some(_) = v.last() {
                                if let Some(condition) = $amongus.exec(cmd) {
                                    save_cmds = condition;
                                }
                            }
                        }
                        _ => {}
                    }
                    if save_cmds {
                        break;
                    }
                    let Some(_) = v.last()
                    else {
                        break;
                    };
                }
            )* 
        }
    }
}
