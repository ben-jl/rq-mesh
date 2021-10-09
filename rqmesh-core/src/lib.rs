use std::path::PathBuf;

#[derive(Debug,Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CapabilityBroadcast {
    capability_type: String
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AgentInitializationContext {
    store_path : PathBuf,
    check_deps_command : String,
    install_deps_command : String
}

