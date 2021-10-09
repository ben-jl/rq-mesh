use rqmesh_core::{AgentInitializationContext, RqMeshError, InitializationErrorKind};
use crate::Agent;
use std::convert::TryFrom;
use std::process::{Command, ExitStatus};
use log::{info,trace,debug,error,warn};

type Result<T> = std::result::Result<T, RqMeshError>;

impl TryFrom<AgentInitializationContext> for Agent {
    type Error = RqMeshError;

    fn try_from(value: AgentInitializationContext) -> Result<Agent> {
        info!("Attempting to create agent from init ctx at location {}", value.store_path().to_string_lossy());

        let check_deps_result = check_dependencies_present(&value);
        if let Err(RqMeshError::InitializationError(InitializationErrorKind::MissingRequiredDependencies { message })) = check_deps_result {
            warn!("Missing required dependencies: {}", &message);
            Ok(())
        } else {
            check_deps_result
        }?;
        todo!()
    }
}

fn check_dependencies_present(ctx: &AgentInitializationContext) -> Result<()> {
    info!("Checking dependencies using context cmd {}", ctx.check_deps_command());

    let raw_cmd = ctx.check_deps_command();
    let split_cmd : Vec<&str> = raw_cmd.split_ascii_whitespace().collect();
    
    trace!("Attempting to get program name from string array {:?}", &split_cmd);
    let program : String = split_cmd.get(0).map_or(Err(RqMeshError::from(InitializationErrorKind::new_invalid_check_deps_cmd(raw_cmd, "Command text must be non-empty"))), |c| Ok(c.to_string()))?;
    trace!("Retrieved program name {}", &program);
    
    trace!("Attempting to get args array (if any) from string array {:?}", &split_cmd);
    let args = &split_cmd[1..];
    trace!("Found args array {:?}", &args);

    trace!("Executing {} with args {:?}", &program, &args);
    let output = Command::new(&program).args(args).output().map_err(|e| RqMeshError::from(InitializationErrorKind::new_invalid_check_deps_cmd(program, format!("{}", e))))?;
    
    match (&output.status, &output.stdout, &output.stderr) {
        (ecode, sout, serr) if !ecode.success() => Err(RqMeshError::from(InitializationErrorKind::new_invalid_check_deps_cmd(raw_cmd, format!("Exit code indicates error: {:?} + {:?}", sout, serr)))),
        (_, sout, _) if sout.iter().filter(|b| !b.is_ascii_whitespace()).count() == 0 => Err(RqMeshError::from(InitializationErrorKind::new_missing_deps(format!("Check dependency command returned empty, ensure dependencies are present")))),
        (_, sout, _) => {
            let sout = String::from_utf8(sout.to_vec()).map_err(|e| RqMeshError::from(InitializationErrorKind::new_invalid_check_deps_cmd(raw_cmd, format!("Error inspecting std out: {}", e))))?;
            info!("Check dependency command returned with valid exit code and output {}", &sout.trim());
            Ok(())
        }
    }?;

    Ok(())
}