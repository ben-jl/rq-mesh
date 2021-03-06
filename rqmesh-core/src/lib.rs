mod protocol;
pub use protocol::{RqMeshFrame, DescribeAgentResponse, DescribeAgentRequest, RqMeshProtocolAction};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

const VERSION: &'static str = "0.1.0";

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CapabilityBroadcast {
    capability_type: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct AgentInitializationContext {
    store_path: PathBuf,
    check_deps_command: String,
    install_deps_command: String,
    version: &'static str,
}

impl AgentInitializationContext {
    pub fn new<T, S1, S2>(
        store_path: T,
        check_deps_command: S1,
        install_deps_command: S2,
    ) -> AgentInitializationContext
    where
        T: Into<PathBuf>,
        S1: Into<String>,
        S2: Into<String>,
    {
        let store_path: PathBuf = store_path.into();
        let check_deps_command: String = check_deps_command.into();
        let install_deps_command: String = install_deps_command.into();
        AgentInitializationContext {
            store_path,
            check_deps_command,
            install_deps_command,
            version: VERSION,
        }
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

    pub fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum RqMeshError {
    InitializationError(InitializationErrorKind),
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

impl std::error::Error for RqMeshError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InitializationErrorKind {
    InvalidStoreLocation {
        store_location: String,
        message: String,
    },
    InvalidCheckDependenciesCommand {
        command: String,
        message: String,
    },
    MissingRequiredDependencies {
        message: String,
    },
    InvalidInstallDependenciesCommand {
        command: String,
        message: String,
    },
    SqliteInitializationError {
        message: String,
    },
}

impl InitializationErrorKind {
    pub fn new_sqlite_init_err<S>(message: S) -> InitializationErrorKind
    where
        S: Into<String>,
    {
        let message = message.into();
        InitializationErrorKind::SqliteInitializationError { message }
    }

    pub fn new_invalid_check_deps_cmd<S1, S2>(command: S1, message: S2) -> InitializationErrorKind
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let command: String = command.into();
        let message: String = message.into();
        InitializationErrorKind::InvalidCheckDependenciesCommand { command, message }
    }

    pub fn new_missing_deps<S>(message: S) -> InitializationErrorKind
    where
        S: Into<String>,
    {
        let message = message.into();
        InitializationErrorKind::MissingRequiredDependencies { message }
    }

    pub fn new_invalid_install_deps_cmd<S1, S2>(command: S1, message: S2) -> InitializationErrorKind
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let command: String = command.into();
        let message: String = message.into();
        InitializationErrorKind::InvalidInstallDependenciesCommand { command, message }
    }

    pub fn new_invalid_store_location<S1, S2>(
        store_location: S1,
        message: S2,
    ) -> InitializationErrorKind
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let store_location = store_location.into();
        let message = message.into();
        InitializationErrorKind::InvalidStoreLocation {
            store_location,
            message,
        }
    }
}

impl std::fmt::Display for InitializationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InitializationErrorKind::InvalidStoreLocation {
                store_location,
                message,
            } => write!(f, "InvalidStoreLocation ({}): {}", store_location, message),
            InitializationErrorKind::InvalidCheckDependenciesCommand { command, message } => {
                write!(
                    f,
                    "InvalidCheckDependenciesCommand ({}): {}",
                    command, message
                )
            }
            InitializationErrorKind::MissingRequiredDependencies { message } => {
                write!(f, "MissingRequiredDependencies: {}", message)
            }
            InitializationErrorKind::InvalidInstallDependenciesCommand { command, message } => {
                write!(
                    f,
                    "InvalidInstallDependenciesCommand ({}): {}",
                    command, message
                )
            }
            InitializationErrorKind::SqliteInitializationError { message } => {
                write!(f, "SqliteInitializationError: {}", message)
            }
        }?;
        Ok(())
    }
}
