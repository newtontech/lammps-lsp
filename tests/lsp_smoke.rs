//! LSP stdio JSON-RPC smoke test.
//!
//! Sends a minimal JSON-RPC `initialize` → `textDocument/didOpen` → `shutdown` → `exit`
//! sequence over stdio to the `lammps-lsp` binary, verifying:
//! - The server starts and responds to `initialize`.
//! - It acknowledges `initialized`.
//! - It processes `textDocument/didOpen` and publishes diagnostics.
//! - It shuts down cleanly.

use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

/// Spawns the `lammps-lsp` binary in stdio mode.
fn spawn_server() -> (Child, ChildStdin, BufReader<ChildStdout>) {
    let bin = env!("CARGO_BIN_EXE_lammps-lsp");
    let mut child = Command::new(bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit()) // let stderr through for debugging
        .spawn()
        .expect("failed to spawn lammps-lsp");

    let stdin = child.stdin.take().expect("no stdin");
    let stdout = child.stdout.take().expect("no stdout");
    let reader = BufReader::new(stdout);

    (child, stdin, reader)
}

/// Send a JSON-RPC message with proper Content-Length header.
fn send_message(stdin: &mut ChildStdin, msg: &serde_json::Value) {
    let body = serde_json::to_string(msg).unwrap();
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    stdin.write_all(header.as_bytes()).unwrap();
    stdin.write_all(body.as_bytes()).unwrap();
    stdin.flush().unwrap();
}

/// Read a single JSON-RPC response from the server.
fn read_message(reader: &mut BufReader<ChildStdout>) -> serde_json::Value {
    // Read headers until empty line
    let mut content_length: usize = 0;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).expect("failed to read header");
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        if let Some(len_str) = line.strip_prefix("Content-Length:") {
            content_length = len_str.trim().parse().unwrap();
        }
    }

    // Read body
    let mut buf = vec![0u8; content_length];
    reader.read_exact(&mut buf).expect("failed to read body");
    serde_json::from_slice(&buf).expect("invalid JSON in response")
}

#[test]
fn lsp_initialize_and_shutdown() {
    let (mut child, mut stdin, mut reader) = spawn_server();

    // 1. Send initialize request
    let init_params = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {}
        }
    });
    send_message(&mut stdin, &init_params);

    let response = read_message(&mut reader);
    assert_eq!(response["id"], 1, "initialize response should have id=1");
    let result = response
        .get("result")
        .expect("initialize should return a result");
    let server_info = result
        .get("serverInfo")
        .expect("result should contain serverInfo");
    assert!(
        server_info["name"].as_str().unwrap().contains("LAMMPS"),
        "server name should mention LAMMPS"
    );

    // 2. Send initialized notification
    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized);

    // Read the log message notification
    let _notif = read_message(&mut reader);

    // 3. Send didOpen with a simple valid LAMMPS script
    let valid_script = "units metal\ndimension 3\nrun 0\n";
    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": "file:///tmp/test.in",
                "languageId": "lammps",
                "version": 1,
                "text": valid_script
            }
        }
    });
    send_message(&mut stdin, &did_open);

    // Read notifications (log + diagnostics)
    // The server sends a log_message and then publishDiagnostics
    let mut got_diagnostics = false;
    for _ in 0..5 {
        let msg = read_message(&mut reader);
        if msg.get("method").and_then(|m| m.as_str()) == Some("textDocument/publishDiagnostics") {
            got_diagnostics = true;
            let diags = &msg["params"]["diagnostics"];
            // For a simple valid script, we may or may not get diagnostics depending on lints
            // Just verify the structure is correct
            assert!(diags.is_array(), "diagnostics should be an array");
            break;
        }
    }
    assert!(
        got_diagnostics,
        "should receive publishDiagnostics notification"
    );

    // 4. Send shutdown request
    let shutdown = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "shutdown"
    });
    send_message(&mut stdin, &shutdown);

    let shutdown_resp = read_message(&mut reader);
    assert_eq!(shutdown_resp["id"], 2, "shutdown response should have id=2");

    // 5. Send exit notification
    let exit = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "exit"
    });
    send_message(&mut stdin, &exit);

    // Give it a moment to exit
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Shutdown stdin to unblock the server
    drop(stdin);

    let exit_status = child.wait().expect("failed to wait for child");
    assert!(
        exit_status.success(),
        "server should exit cleanly, got {exit_status:?}"
    );
}

#[test]
fn lsp_diagnostics_on_invalid_script() {
    let (mut child, mut stdin, mut reader) = spawn_server();

    // Initialize
    let init_params = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {}
        }
    });
    send_message(&mut stdin, &init_params);
    let _init_resp = read_message(&mut reader);

    // Initialized
    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized);
    let _log = read_message(&mut reader);

    // Open a file with obvious syntax errors
    let invalid_script = "this is not a valid lammps command\nbogus_xyz 1 2 3\n";
    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": "file:///tmp/bad.in",
                "languageId": "lammps",
                "version": 1,
                "text": invalid_script
            }
        }
    });
    send_message(&mut stdin, &did_open);

    // Collect messages until we get publishDiagnostics
    let mut diagnostics_json: Option<serde_json::Value> = None;
    for _ in 0..10 {
        let msg = read_message(&mut reader);
        if msg.get("method").and_then(|m| m.as_str()) == Some("textDocument/publishDiagnostics") {
            diagnostics_json = Some(msg["params"]["diagnostics"].clone());
            break;
        }
    }

    let diags = diagnostics_json.expect("should receive publishDiagnostics");
    let diags_arr = diags.as_array().expect("diagnostics should be array");
    assert!(
        !diags_arr.is_empty(),
        "invalid script should produce at least one diagnostic"
    );

    // Verify LSP diagnostic structure
    for diag in diags_arr {
        assert!(diag.get("range").is_some(), "diagnostic should have range");
        assert!(
            diag.get("message").is_some(),
            "diagnostic should have message"
        );
        assert!(
            diag.get("severity").is_some(),
            "diagnostic should have severity"
        );
    }

    // Cleanup
    let shutdown = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "shutdown"
    });
    send_message(&mut stdin, &shutdown);
    let _shutdown_resp = read_message(&mut reader);

    let exit = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "exit"
    });
    send_message(&mut stdin, &exit);
    drop(stdin);
    std::thread::sleep(std::time::Duration::from_millis(500));
    let _ = child.wait();
}
