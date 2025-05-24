use std::{fs::File, path::Path};
use std::io::Write; 

use crate::{banner, scanner::ScanResult};
use prettytable::{Table, row, cell};
use serde_json;

pub fn print_table(results: &[ScanResult]) {
    let mut table = Table::new();
    table.add_row(row!["Port", "Open", "Banner (truncated)"]);

    for r in results.iter().filter(|r| r.open) {
        let banner_preview = r.banner.as_ref()
            .map(|b| if b.len() > 40 { &b[..40] } else { &b[..] })
            .unwrap_or("");
        table.add_row(row![r.port, "Yes", banner_preview]);
    }

    table.printstd();
}

pub fn print_json(results: &[ScanResult], filepath: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(results).unwrap_or_else(|_| "[]".to_string());

    let path = Path::new(filepath);
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Escapes HTML special characters in a string.
fn encode_text(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn export_html(results: &[ScanResult], output_path: &str) -> std::io::Result<()> {
    let mut file = File::create(output_path)?;

    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Scan Results</title>
<style>
  body { font-family: Arial, sans-serif; padding: 20px; background-color: #f5f5f5; }
  table { border-collapse: collapse; width: 100%; background-color: #fff; }
  th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
  th { background-color: #4CAF50; color: white; }
  tr:nth-child(even) { background-color: #f2f2f2; }
  .banner { font-family: monospace; white-space: pre-wrap; }
</style>
</head>
<body>
<h1>Open Ports Scan Results</h1>
<table>
<thead>
<tr><th>Port</th><th>Status</th><th>Banner (truncated)</th></tr>
</thead>
<tbody>
"#);

    for r in results.iter().filter(|r| r.open) {
        let banner = r.banner.as_ref().map(|b| {
            let truncated = if b.len() > 60 { &b[..60] } else { b };
            encode_text(truncated)
        }).unwrap_or_default();

        html.push_str(&format!(
            "<tr><td>{}</td><td>Open</td><td class=\"banner\">{}</td></tr>\n",
            r.port, banner
        ));
    }

    html.push_str(r#"
</tbody>
</table>
</body>
</html>
"#);

    file.write_all(html.as_bytes())
}

