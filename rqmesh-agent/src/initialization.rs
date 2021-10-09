use crate::Agent;
use log::{error, info, trace, warn};
use rqmesh_core::{AgentInitializationContext, InitializationErrorKind, RqMeshError};
use rusqlite::{params, Connection};
use std::convert::TryFrom;
use std::process::Command;

type Result<T> = std::result::Result<T, RqMeshError>;

impl TryFrom<AgentInitializationContext> for Agent {
    type Error = RqMeshError;

    fn try_from(value: AgentInitializationContext) -> Result<Agent> {
        info!(
            "Attempting to create agent from init ctx at location {}",
            value.store_path().to_string_lossy()
        );

        let check_deps_result = check_dependencies_present(&value);
        if let Err(RqMeshError::InitializationError(
            InitializationErrorKind::MissingRequiredDependencies { message },
        )) = check_deps_result
        {
            warn!("Missing required dependencies: {}", &message);
            try_install_missing_dependencies(&value)?;

            check_dependencies_present(&value)?;
            Ok(())
        } else {
            check_deps_result
        }?;

        check_store_path(&value)?;
        let conn = Connection::open(value.store_path()).map_err(|e| {
            RqMeshError::from(InitializationErrorKind::new_sqlite_init_err(format!(
                "{}",
                e
            )))
        })?;
        let conn = validate_or_initialize_sqlite_connection(&value, conn)?;
        Ok(Agent { connection: conn })
    }
}

fn check_dependencies_present(ctx: &AgentInitializationContext) -> Result<()> {
    info!(
        "Checking dependencies using context cmd {}",
        ctx.check_deps_command()
    );

    let raw_cmd = ctx.check_deps_command();
    let split_cmd: Vec<&str> = raw_cmd.split_ascii_whitespace().collect();

    trace!(
        "Attempting to get program name from string array {:?}",
        &split_cmd
    );
    let program: String = split_cmd.get(0).map_or(
        Err(RqMeshError::from(
            InitializationErrorKind::new_invalid_check_deps_cmd(
                raw_cmd,
                "Command text must be non-empty",
            ),
        )),
        |c| Ok(c.to_string()),
    )?;
    trace!("Retrieved program name {}", &program);

    trace!(
        "Attempting to get args array (if any) from string array {:?}",
        &split_cmd
    );
    let args = &split_cmd[1..];
    trace!("Found args array {:?}", &args);

    trace!("Executing {} with args {:?}", &program, &args);
    let output = Command::new(&program).args(args).output().map_err(|e| {
        RqMeshError::from(InitializationErrorKind::new_invalid_check_deps_cmd(
            program,
            format!("{}", e),
        ))
    })?;

    match (&output.status, &output.stdout, &output.stderr) {
        (ecode, sout, serr) if !ecode.success() => Err(RqMeshError::from(
            InitializationErrorKind::new_invalid_check_deps_cmd(
                raw_cmd,
                format!("Exit code indicates error: {:?} + {:?}", sout, serr),
            ),
        )),
        (_, sout, _) if sout.iter().filter(|b| !b.is_ascii_whitespace()).count() == 0 => Err(
            RqMeshError::from(InitializationErrorKind::new_missing_deps(format!(
                "Check dependency command returned empty, ensure dependencies are present"
            ))),
        ),
        (_, sout, _) => {
            let sout = String::from_utf8(sout.to_vec()).map_err(|e| {
                RqMeshError::from(InitializationErrorKind::new_invalid_check_deps_cmd(
                    raw_cmd,
                    format!("Error inspecting std out: {}", e),
                ))
            })?;
            info!(
                "Check dependency command returned with valid exit code and output {}",
                &sout.trim()
            );
            Ok(())
        }
    }?;

    Ok(())
}

fn try_install_missing_dependencies(ctx: &AgentInitializationContext) -> Result<()> {
    info!(
        "Attempting to install missing dependencies using context cmd {}",
        ctx.install_deps_command()
    );

    let raw_cmd = ctx.install_deps_command();
    let split_cmd: Vec<&str> = raw_cmd.split_ascii_whitespace().collect();

    trace!(
        "Attempting to get program name from string array {:?}",
        &split_cmd
    );
    let program: String = split_cmd.get(0).map_or(
        Err(RqMeshError::from(
            InitializationErrorKind::new_invalid_install_deps_cmd(
                raw_cmd,
                "Command text must be non-empty",
            ),
        )),
        |c| Ok(c.to_string()),
    )?;
    trace!("Retrieved program name {}", &program);

    trace!(
        "Attempting to get args array (if any) from string array {:?}",
        &split_cmd
    );
    let args = &split_cmd[1..];
    trace!("Found args array {:?}", &args);

    trace!("Executing {} with args {:?}", &program, &args);
    let output = Command::new(&program).args(args).output().map_err(|e| {
        RqMeshError::from(InitializationErrorKind::new_invalid_install_deps_cmd(
            program,
            format!("{}", e),
        ))
    })?;

    match (&output.status, &output.stdout, &output.stderr) {
        (ecode, sout, serr) if !ecode.success() => Err(RqMeshError::from(
            InitializationErrorKind::new_invalid_install_deps_cmd(
                raw_cmd,
                format!("Exit code indicates error: {:?} + {:?}", sout, serr),
            ),
        )),
        (_, sout, _) => {
            let sout = String::from_utf8(sout.to_vec()).map_err(|e| {
                RqMeshError::from(InitializationErrorKind::new_invalid_install_deps_cmd(
                    raw_cmd,
                    format!("Error inspecting std out: {}", e),
                ))
            })?;
            info!(
                "Install dependency command returned with valid exit code and output {}",
                &sout.trim()
            );
            Ok(())
        }
    }?;

    Ok(())
}

fn check_store_path(ctx: &AgentInitializationContext) -> Result<()> {
    info!(
        "Ensuring store location {} exists and is reachable",
        ctx.store_path().to_str().unwrap_or("NONE")
    );

    let pbuf = ctx.store_path();
    trace!(
        "Checking if {} already exists",
        pbuf.to_str().unwrap_or("NONE")
    );
    if !pbuf.exists() {
        trace!(
            "File {} not found, attempting to create",
            pbuf.to_str().unwrap_or("NONE")
        );
        let dir = pbuf.parent().ok_or(RqMeshError::from(
            InitializationErrorKind::new_invalid_store_location(
                pbuf.to_str().unwrap_or("NONE"),
                format!(
                    "Could not find directory {}",
                    pbuf.to_str().unwrap_or("NONE")
                ),
            ),
        ))?;
        trace!(
            "Ensuring parent directory {} exists",
            dir.to_str().unwrap_or("NONE")
        );
        if !dir.exists() {
            error!(
                "Store location directory {} not found, try creating and restarting the agent",
                dir.to_str().unwrap_or("NONE")
            );
            return Err(RqMeshError::from(
                InitializationErrorKind::new_invalid_store_location(
                    pbuf.to_str().unwrap_or("NONE"),
                    format!("Directory {} not found", dir.to_str().unwrap_or("NONE")),
                ),
            ));
        }
    } else {
        trace!(
            "{} already exists, no action necessary",
            pbuf.to_str().unwrap_or("NONE")
        );
    }

    Ok(())
}

fn validate_or_initialize_sqlite_connection(
    ctx: &AgentInitializationContext,
    conn: Connection,
) -> Result<Connection> {
    trace!("Ensuring agent_details table exists");
    conn.execute("
        CREATE TABLE IF NOT EXISTS agent_details (version VARCHAR(10) NOT NULL, store_location NVARCHAR(1024) NOT NULL, initialized_at VARCHAR(100) NOT NULL, UNIQUE(version, store_location));
       
        ",
          []).map_err(|e| RqMeshError::from(InitializationErrorKind::new_sqlite_init_err(format!("Error creating agent_details table: {}", e))))?;

    trace!("Ensuring agent_details table is populated with current version and details");
    conn.execute("INSERT OR IGNORE INTO agent_details (version, store_location, initialized_at) VALUES (?1, ?2, datetime('now'));", 
        params![ctx.version(), ctx.store_path().to_str().unwrap_or("NA")]).map_err(|e| RqMeshError::from(InitializationErrorKind::new_sqlite_init_err(format!("Error inserting into agent_details table: {}", e))))?;
    Ok(conn)
}
