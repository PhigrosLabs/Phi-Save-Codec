use multi_value_gen::parse;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use syn::parse_file;
use walrus::ValType;

fn extract_functions_from_c_api(
    c_api_path: &str,
) -> Result<HashMap<String, Vec<ValType>>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(c_api_path)?;
    let file = parse_file(&content)?;
    let mut funcs: HashMap<String, Vec<ValType>> = HashMap::new();
    funcs.insert(
        "psc_get_last_error".to_string(),
        vec![ValType::I32, ValType::I32],
    ); // 固定的

    for item in file.items {
        if let syn::Item::Macro(item_macro) = item {
            if item_macro.mac.path.is_ident("impl_c_api") {
                let tokens = &item_macro.mac.tokens;
                let tokens_str = tokens.to_string();

                let params_str = tokens_str.trim_start_matches('(').trim_end_matches(')');
                let params: Vec<&str> = params_str.split(',').map(|s: &str| s.trim()).collect();

                if params.len() >= 4 {
                    let parse_fn = params[2];
                    let build_fn = params[3];

                    funcs.insert(parse_fn.to_string(), vec![ValType::I32, ValType::I32]);
                    funcs.insert(build_fn.to_string(), vec![ValType::I32, ValType::I32]);

                    println!("提取函数: {} {}", parse_fn, build_fn);
                }
            }
        }
    }

    Ok(funcs)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .args([
            "build",
            "--lib",
            "-p",
            "phi_save_codec",
            "--features",
            "c_abi",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()?;

    if !status.success() {
        eprintln!("cargo build 失败，退出程序");
        std::process::exit(1);
    }

    let c_api_path = "./app/src/c_api.rs";
    let funcs = match extract_functions_from_c_api(c_api_path) {
        Ok(f) => {
            if f.is_empty() {
                eprintln!("警告：从 {} 未提取到任何函数", c_api_path);
                HashMap::new()
            } else {
                f
            }
        }
        Err(e) => {
            eprintln!("读取或解析 {} 出错: {}", c_api_path, e);
            HashMap::new()
        }
    };

    println!("找到 {} 个API函数", funcs.len());

    let wasm_file = "./target/wasm32-unknown-unknown/release/phi_save_codec.wasm";
    let wasm_bytes = fs::read(wasm_file)?;

    match parse(wasm_bytes, funcs) {
        Ok(processed_wasm) => {
            let output_dir = "./output/";
            fs::create_dir_all(output_dir)?;

            let output_path = format!("{}phi_save_codec.wasm", output_dir);
            fs::write(&output_path, processed_wasm)?;

            println!("保存到: {}", output_path);
        }
        Err(e) => {
            eprintln!("处理WASM文件时出错: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
