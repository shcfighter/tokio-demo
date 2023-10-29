use std::collections::HashMap;

use super::Argument;


pub struct Message {
    pub message : Vec<String>,
    pub topic: String,
    pub len: u32,
}

pub struct Topic {
    pub topic : String,
    pub partitions : u32,
    pub group : Vec<String>,
    pub replication: u8,
}

pub struct Metadata {
    pub topic_map : HashMap<String, Topic>,
}

impl Topic {
    // pub fn new(args: &Vec<Argument>) -> Self {
    //     for arg in args.iter() {
    //         let t = match *arg {
    //             Argument::Topic { topic } => topic,
    //             _ => panic!(""),
    //         };
    //     }
    //     //topic: String, partitions: u32, group: Vec<String>, replication: u8;
    //     TopicInfo {
    //         topic, 
    //         partitions,
    //         group,
    //         replication,
    //     }
    // }


}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            topic_map: HashMap::new(),
        }
    }
}