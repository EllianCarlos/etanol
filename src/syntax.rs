use tower_lsp::{lsp_types::*};
use tokio::process::Command;
use std::process::Stdio;
use regex::Regex;

use crate::server::KotlinLsp;
use crate::util::create_temp_file;

pub async fn check_syntax(lsp: &KotlinLsp, code: &str, uri: &str) {
    let temp_file = create_temp_file(lsp, code).await;

    if let Some(temp_path) = temp_file {
        let output = match Command::new("kotlinc")
            .arg("-script")
            .arg(temp_path.to_str().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
        {
            Ok(output) => output,
            Err(e) => {
                lsp.client.log_message(MessageType::ERROR, format!("Failed to start kotlinc: {}", e)).await;
                return;
            }
        };

        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut diagnostics = vec![];

        if !stderr.is_empty() {
            let error_regex =
                Regex::new(r"(?P<file>.*):(?P<line>\d+):(?P<column>\d+):\s*error:\s*(?P<message>.+)").unwrap();

            for cap in error_regex.captures_iter(&stderr) {
                if let (Some(line), Some(column), Some(message)) =
                    (cap.name("line"), cap.name("column"), cap.name("message"))
                {
                    let diagnostic = Diagnostic {
                        range: Range {
                            start: Position {
                                line: line.as_str().parse::<u32>().unwrap_or(0) - 1,
                                character: column.as_str().parse::<u32>().unwrap_or(0) - 1,
                            },
                            end: Position {
                                line: line.as_str().parse::<u32>().unwrap_or(0) - 1,
                                character: column.as_str().parse::<u32>().unwrap_or(0) + 5,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: message.as_str().to_string(),
                        ..Diagnostic::default()
                    };

                    diagnostics.push(diagnostic);
                }
            }
        }

        lsp.client.publish_diagnostics(Url::parse(uri).unwrap(), diagnostics, None).await;
    }
}
