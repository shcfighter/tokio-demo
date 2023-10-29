use std::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SocketState {
    None,
    CreateTopic,    //创建 Topic
    List,
    Describe,
    Alter,
    GetOffset,
    DeleteTopic,
    Produce,
    Consume,
    DeleteGroup,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArgumentName {
    None,
    Topic,
    Partitions,
    FromBeginning,
    Offset,
    Group,
    ReplicationFactor,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Argument {
    None,
    End,
    Topic {
        topic: String
    },
    Partitions {
        partitions: u16
    },
    FromBeginning {
        from_beginning: u16
    },
    Offset {
        offset: String
    },
    Group {
        group: String
    },
    ReplicationFactor {
        replication_factor: u16
    },
}

#[derive(Debug, Clone)]
pub struct Token {
    pub state: SocketState,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Input<'a> {
    pub buf: &'a [u8; 1024],
    pub len: usize,
    pub current: usize,
}

impl<'a> Input<'a> {

    pub fn new(buf: &'a [u8; 1024], len: usize) -> Self {
        Input { 
            buf,
            len, 
            current: 0,
        }
    }

    //pub fn analysis(&mut self) -> Result<Token, Box<dyn Error>> {
    pub fn analysis(&mut self) -> Result<Token, String> {

        let socket_state: SocketState = self.socket_state();
        if socket_state == SocketState::None {
            return Err(String::from("bad command state"));
        }
        //println!("socket_state:{:?}", socket_state);
        
        let args: Vec<Argument> = self.argument(); 
        for arg in args.iter() {
            if *arg == Argument::None {
                return Err(String::from("bad args"));
            }
        }

        // for arg in args.iter() {
        //     println!("argument:{:?}", arg);
        // }


        Ok(Token {
            state: socket_state,
            args: args,
        })
    }

    pub fn argument(&mut self) -> Vec<Argument> {
        let mut args: Vec<Argument> = Vec::new();

        loop {
            if (self.buf[self.current] as char).eq(&'\r') && (self.buf[self.current + 1] as char).eq(&'\n') {
                args.push(Argument::End);
                break;
            }

            if !Self::is_whitespace(self.buf[self.current] as char) {
                args.push(Argument::None);
                break;
            }
            self.current = self.current + 1;

            let arg_type = self.argument_key();
            if arg_type == ArgumentName::None {
                args.push(Argument::None);
                break;
            }

            if !Self::is_whitespace(self.buf[self.current] as char) {
                args.push(Argument::None);
                break;
            }
            self.current = self.current + 1;

            let value: String = self.argument_value();

            let arg = match arg_type {
                ArgumentName::Topic => Argument::Topic {
                    topic: value,
                },
                ArgumentName::Partitions => Argument::Partitions { 
                    partitions: value.parse().unwrap(),
                },
                ArgumentName::FromBeginning => Argument::FromBeginning { 
                    from_beginning: value.parse().unwrap(),
                },
                ArgumentName::Group => Argument::Group {
                    group: value,
                },
                ArgumentName::Offset => Argument::Offset {
                    offset: value,
                },
                ArgumentName::ReplicationFactor => Argument::ReplicationFactor {
                    replication_factor: value.parse().unwrap(),
                },
                _ => Argument::None,
            };

            args.push(arg);
        }

        args
    }

    pub fn argument_value(&mut self) -> String {
        
        let mut value: String = String::new();
        while !Self::is_whitespace(self.buf[self.current] as char) {
            value.push(self.buf[self.current] as char);
            self.current = self.current + 1;
        }
        value
    }

    pub fn argument_key(&mut self) -> ArgumentName {
        
        if Self::is_cross(self.buf[self.current]) && Self::is_cross(self.buf[self.current + 1]) {
            self.current = self.current + 2;

            let mut argument = String::new();
            while !Self::is_whitespace(self.buf[self.current] as char) {
                argument.push(self.buf[self.current] as char);
                self.current = self.current + 1;
            }

            return match argument.as_str() {
                "topic" => ArgumentName::Topic,
                "partitions" => ArgumentName::Partitions,
                "replication-factor" => ArgumentName::ReplicationFactor,
                "from-beginning" => ArgumentName::FromBeginning,
                "offset" => ArgumentName::Offset,
                "group" => ArgumentName::Group,
                _ => ArgumentName::None,
            };
        }

        ArgumentName::None
    }

    pub fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }
    
    pub fn is_cross(b: u8) -> bool {
        (b as char).eq(&'-')
    }

    // pub fn is_alphabetic(c: char) -> bool {
    //     c.is_alphabetic()
    // }

    pub fn socket_state(&mut self) -> SocketState {
        let mut socket_state = String::new();
        while self.current < self.len && !Self::is_whitespace(self.buf[self.current] as char) {
            socket_state.push(self.buf[self.current] as char);
            self.current = self.current + 1;
        }

        match socket_state.as_str() {
            "create_topic" => SocketState::CreateTopic,
            "list" => SocketState::List,
            "describe" => SocketState::Describe,
            "alter" => SocketState::Alter,
            "get_offset" => SocketState::GetOffset,
            "delete_topic" => SocketState::DeleteTopic,
            "produce" => SocketState::Produce,
            "consume" => SocketState::Consume,
            "delete_group" => SocketState::DeleteGroup,
            _ => SocketState::None,
        }
    }

}