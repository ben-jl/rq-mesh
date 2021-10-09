use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct RqMeshFrame<T> where T : RqMeshProtocolAction {
    contents: T,
    requestor: String
}

pub trait RqMeshProtocolAction: Serialize {
    type ResponseType : Serialize;
}

#[derive(Debug,Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Serialize,Deserialize,Default)]
pub struct DescribeAgentRequest { }

#[derive(Debug,Clone,Eq,PartialEq,Ord,PartialOrd,Serialize,Deserialize)]
pub struct DescribeAgentResponse {
    version: String,
    storage_location: String,
    initialized_at: String
}

impl DescribeAgentResponse {
    pub fn new<S1, S2, S3>(version: S1, storage_location: S2, initialized_at: S3) -> DescribeAgentResponse where S1: Into<String>, S2: Into<String>, S3: Into<String> {
        let version = version.into();
        let storage_location = storage_location.into();
        let initialized_at = initialized_at.into();
        DescribeAgentResponse { version, storage_location, initialized_at }
    }
}

impl RqMeshProtocolAction for DescribeAgentRequest {
    type ResponseType = DescribeAgentResponse;
}