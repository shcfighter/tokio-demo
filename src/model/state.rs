
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum ArgumentName {
    None,
    Topic,
    Partitions,
    FromBeginning,
    Offset,
    Group,
    ReplicationFactor,
}

#[derive(Debug, PartialEq)]
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