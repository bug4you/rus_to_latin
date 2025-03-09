use std::fs;
use std::path::Path;

fn main() {
    let package_json_path = "pkg/package.json";
    let npmrc_path = "pkg/.npmrc";

    let mut package_json: serde_json::Value = match fs::read_to_string(package_json_path) {
        Ok(content) => serde_json::from_str(&content).expect("Invalid JSON format"),
        Err(_) => serde_json::json!({}),
    };

    package_json["scripts"] = serde_json::json!({
        "publish": "cd pkg && npm publish --access public"
    });

    fs::write(package_json_path, serde_json::to_string_pretty(&package_json).unwrap())
        .expect("Failed to write package.json");

    let npmrc_content = "registry=https://registry.npmjs.org/\n";

    if !Path::new(npmrc_path).exists() {
        fs::write(npmrc_path, npmrc_content)
            .expect("Failed to write .npmrc file");
    }
}
