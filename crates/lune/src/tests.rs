use std::env::set_current_dir;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use console::set_colors_enabled;
use console::set_colors_enabled_stderr;
use tokio::fs::read_to_string;

use lune_utils::path::clean_path_and_make_absolute;

use crate::Runtime;

const ARGS: &[&str] = &["Foo", "Bar"];

macro_rules! create_tests {
    ($($name:ident: $value:expr,)*) => { $(
        #[tokio::test(flavor = "multi_thread")]
        async fn $name() -> Result<ExitCode> {
            // We need to change the current directory to the workspace root since
            // we are in a sub-crate and tests would run relative to the sub-crate
            let workspace_dir_str = format!("{}/../../", env!("CARGO_MANIFEST_DIR"));
            let workspace_dir = clean_path_and_make_absolute(PathBuf::from(workspace_dir_str));
            set_current_dir(&workspace_dir)?;

            // Disable styling for stdout and stderr since
            // some tests rely on output not being styled
            set_colors_enabled(false);
            set_colors_enabled_stderr(false);

            // The rest of the test logic can continue as normal
            let full_name = format!("{}/tests/{}.luau", workspace_dir.display(), $value);
            let script = read_to_string(&full_name).await?;
            let mut lune = Runtime::new()
                .with_jit(true)
                .with_args(
                    ARGS
                        .clone()
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                );
            let script_name = full_name
				.trim_end_matches(".luau")
				.trim_end_matches(".lua")
				.to_string();
            let (exit_code, _) = lune.run(&script_name, &script).await?;
            Ok(ExitCode::from(exit_code))
        }
    )* }
}

#[cfg(any(
    feature = "std-datetime",
    feature = "std-fs",
    feature = "std-luau",
    feature = "std-net",
    feature = "std-process",
    feature = "std-regex",
    feature = "std-serde",
    feature = "std-stdio",
    feature = "std-task",
))]
create_tests! {
    require_aliases: "require/tests/aliases",
    require_async: "require/tests/async",
    require_async_concurrent: "require/tests/async_concurrent",
    require_async_sequential: "require/tests/async_sequential",
    require_builtins: "require/tests/builtins",
    require_children: "require/tests/children",
    require_init: "require/tests/init",
    require_invalid: "require/tests/invalid",
    require_multi_ext: "require/tests/multi_ext",
    require_nested: "require/tests/nested",
    require_parents: "require/tests/parents",
    require_siblings: "require/tests/siblings",
    require_state: "require/tests/state",

    global_g_table: "globals/_G",
    global_version: "globals/_VERSION",
    global_coroutine: "globals/coroutine",
    global_error: "globals/error",
    global_pcall: "globals/pcall",
    global_type: "globals/type",
    global_typeof: "globals/typeof",
    global_warn: "globals/warn",
}

#[cfg(feature = "std-datetime")]
create_tests! {
    datetime_format_local_time: "datetime/formatLocalTime",
    datetime_format_universal_time: "datetime/formatUniversalTime",
    datetime_from_iso_date: "datetime/fromIsoDate",
    datetime_from_local_time: "datetime/fromLocalTime",
    datetime_from_universal_time: "datetime/fromUniversalTime",
    datetime_from_unix_timestamp: "datetime/fromUnixTimestamp",
    datetime_now: "datetime/now",
    datetime_to_iso_date: "datetime/toIsoDate",
    datetime_to_local_time: "datetime/toLocalTime",
    datetime_to_universal_time: "datetime/toUniversalTime",
}

#[cfg(feature = "std-fs")]
create_tests! {
    fs_files: "fs/files",
    fs_copy: "fs/copy",
    fs_dirs: "fs/dirs",
    fs_metadata: "fs/metadata",
    fs_move: "fs/move",
}

#[cfg(feature = "std-luau")]
create_tests! {
    luau_compile: "luau/compile",
    luau_load: "luau/load",
    luau_options: "luau/options",
    luau_safeenv: "luau/safeenv",
}

#[cfg(feature = "std-net")]
create_tests! {
    net_request_codes: "net/request/codes",
    net_request_compression: "net/request/compression",
    net_request_methods: "net/request/methods",
    net_request_query: "net/request/query",
    net_request_redirect: "net/request/redirect",
    net_url_encode: "net/url/encode",
    net_url_decode: "net/url/decode",
    net_serve_requests: "net/serve/requests",
    net_serve_websockets: "net/serve/websockets",
    net_socket_basic: "net/socket/basic",
    net_socket_wss: "net/socket/wss",
    net_socket_wss_rw: "net/socket/wss_rw",
}

#[cfg(feature = "std-process")]
create_tests! {
    process_args: "process/args",
    process_cwd: "process/cwd",
    process_env: "process/env",
    process_exit: "process/exit",
    process_exec_async: "process/exec/async",
    process_exec_basic: "process/exec/basic",
    process_exec_cwd: "process/exec/cwd",
    process_exec_no_panic: "process/exec/no_panic",
    process_exec_shell: "process/exec/shell",
    process_exec_stdin: "process/exec/stdin",
    process_exec_stdio: "process/exec/stdio",
    process_spawn_non_blocking: "process/create/non_blocking",
    process_spawn_status: "process/create/status",
    process_spawn_stream: "process/create/stream",
}

#[cfg(feature = "std-regex")]
create_tests! {
    regex_general: "regex/general",
    regex_metamethods: "regex/metamethods",
    regex_replace: "regex/replace",
}

#[cfg(feature = "std-serde")]
create_tests! {
    serde_compression_files: "serde/compression/files",
    serde_compression_roundtrip: "serde/compression/roundtrip",
    serde_json_decode: "serde/json/decode",
    serde_json_encode: "serde/json/encode",
    serde_toml_decode: "serde/toml/decode",
    serde_toml_encode: "serde/toml/encode",
    serde_hashing_hash: "serde/hashing/hash",
    serde_hashing_hmac: "serde/hashing/hmac",
}

#[cfg(feature = "std-stdio")]
create_tests! {
    stdio_format: "stdio/format",
    stdio_color: "stdio/color",
    stdio_style: "stdio/style",
    stdio_write: "stdio/write",
    stdio_ewrite: "stdio/ewrite",
}

#[cfg(feature = "std-task")]
create_tests! {
    task_cancel: "task/cancel",
    task_defer: "task/defer",
    task_delay: "task/delay",
    task_spawn: "task/spawn",
    task_wait: "task/wait",
}
