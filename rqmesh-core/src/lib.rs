use std::path::{Path, PathBuf};

#[derive(Debug,Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CapabilityBroadcast {
    capability_type: String
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct AgentInitializationContext {
    store_path : PathBuf,
    check_deps_command : String,
    install_deps_command : String
}

impl AgentInitializationContext {
    pub fn new<T,S1,S2>(store_path: T, check_deps_command: S1, install_deps_command: S2) -> AgentInitializationContext where T : Into<PathBuf>, S1 : Into<String>, S2 : Into<String> {
        let store_path : PathBuf = store_path.into();
        let check_deps_command : String = check_deps_command.into();
        let install_deps_command : String = install_deps_command.into();
        AgentInitializationContext { store_path, check_deps_command, install_deps_command }
    }

    pub fn store_path(&self) -> &PathBuf {
        &self.store_path
    }

    pub fn check_deps_command(&self) -> &str {
        &self.check_deps_command
    }

    pub fn install_deps_command(&self) -> &str {
        &self.install_deps_command
    }
}

#[derive(Debug,Clone,PartialEq, PartialOrd, Eq, Ord)]
pub enum RqMeshError {
    InitializationError(InitializationErrorKind)
}

impl From<InitializationErrorKind> for RqMeshError {
    fn from(value: InitializationErrorKind) -> RqMeshError {
        RqMeshError::InitializationError(value)
    }
}

impl std::fmt::Display for RqMeshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RqMeshError::InitializationError(i) => write!(f, "{}", i),
        }?;
        Ok(())
    }
}

impl std::error::Error for RqMeshError { }

#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub enum InitializationErrorKind {
    InvalidStoreLocation { store_location: String, message: String },
    InvalidCheckDependenciesCommand { command: String, message: String },
    MissingRequiredDependencies { message : String }
}

impl InitializationErrorKind {
    pub fn new_invalid_check_deps_cmd<S1, S2>(command: S1, message: S2) -> InitializationErrorKind where S1 : Into<String>, S2: Into<String> {
        let command : String = command.into();
        let message : String = message.into();
        InitializationErrorKind::InvalidCheckDependenciesCommand { command, message }
    }

    pub fn new_missing_deps<S>(message : S) -> InitializationErrorKind where S : Into<String> {
        let message = message.into();
        InitializationErrorKind::MissingRequiredDependencies { message }
    }
}

impl std::fmt::Display for InitializationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InitializationErrorKind::InvalidStoreLocation { store_location, message } => write!(f,"InvalidStoreLocation ({}): {}", store_location, message),
            InitializationErrorKind::InvalidCheckDependenciesCommand { command, message } => write!(f, "InvalidCheckDependenciesCommand ({}): {}", command, message),
            InitializationErrorKind::MissingRequiredDependencies { message  } => write!(f, "MissingRequiredDependencies: {}", message)
        }?;
        Ok(())
    }
}