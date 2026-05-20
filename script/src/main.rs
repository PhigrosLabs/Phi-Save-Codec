use multi_value_gen::parse;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use syn::{Item, parse_file};

use walrus::ValType;

fn extract_functions_from_c_api(
    c_api_path: &str,
) -> Result<HashMap<String, Vec<ValType>>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(c_api_path)?;
    let file = parse_file(&content)?;

    let mut funcs: HashMap<String, Vec<ValType>> = HashMap::new();

    for item in file.items {
        if let Item::Macro(item_macro) = item {
            let path = item_macro
                .mac
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string());

            if path.as_deref() == Some("impl_c_api") {
                let tokens = item_macro.mac.tokens.to_string();

                let inner = tokens.trim_start_matches('(').trim_end_matches(')').trim();

                let params: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

                if params.len() >= 3 {
                    let parse_fn = params[1];
                    let build_fn = params[2];

                    let sig = vec![ValType::I32, ValType::I32, ValType::I32];

                    funcs.insert(parse_fn.to_string(), sig.clone());
                    funcs.insert(build_fn.to_string(), sig);
                } else {
                    println!("[WARN] macro 参数不足: {}", tokens);
                }
            }
        }
    }

    Ok(funcs)
}

fn build_wasm() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .args([
            "build",
            "--lib",
            "-p",
            "phi_save_codec_c_api",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Build WASM ===");
    build_wasm()?;

    let c_api_path = "./c_api/src/lib.rs";

    println!("=== Extract C API ===");
    let funcs = extract_functions_from_c_api(c_api_path)?;

    println!("找到 {} 个API函数", funcs.len());

    if funcs.is_empty() {
        return Err("没有需要转换的函数".into());
    }

    let wasm_file = "./target/wasm32-unknown-unknown/release/phi_save_codec_c_api.wasm";
    let wasm_bytes = fs::read(wasm_file)?;

    println!("=== Processing WASM ===");
    let processed_wasm = parse(wasm_bytes, funcs)?;

    let output_dir = "./output";
    fs::create_dir_all(output_dir)?;

    let output_path = format!("{}/phi_save_codec.wasm", output_dir);
    fs::write(&output_path, processed_wasm)?;

    println!("输出完成: {}", output_path);

    Ok(())
}
