use std::fs;
use std::path::PathBuf;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ProgramInfo {
    program_id: String,
    instructions: Vec<InstructionInfo>,
    accounts: Vec<AccountInfo>,
    errors: Vec<ErrorInfo>,
    structs: Vec<StructInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstructionInfo {
    name: String,
    arguments: Vec<ArgumentInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArgumentInfo {
    name: String,
    type_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountInfo {
    name: String,
    fields: Vec<FieldInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FieldInfo {
    name: String,
    type_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorInfo {
    name: String,
    code: u32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StructInfo {
    name: String,
    fields: Vec<FieldInfo>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "dump_info" {
        dump_program_info();
    } else {
        // 默认行为：什么都不做
        println!("Solana Swap Program - Use 'dump_info' command to export program definition");
    }
}

fn dump_program_info() {
    let project_root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let lib_path = project_root.join("programs/swap-program/src/lib.rs");

    let lib_content = fs::read_to_string(&lib_path).unwrap_or_else(|_| {
        eprintln!("Warning: Could not read lib.rs at {:?}", lib_path);
        String::new()
    });

    let program_info = parse_program_info(&lib_content);

    println!("{}", serde_json::to_string_pretty(&program_info).unwrap());
}

fn parse_program_info(content: &str) -> ProgramInfo {
    let mut program_info = ProgramInfo {
        program_id: String::new(),
        instructions: Vec::new(),
        accounts: Vec::new(),
        errors: Vec::new(),
        structs: Vec::new(),
    };

    // 解析 program_id
    if let Some(captures) = regex::Regex::new(r#"declare_id!\("([^"]+)"\)"#).unwrap().captures(content) {
        if let Some(id) = captures.get(1) {
            program_info.program_id = id.as_str().to_string();
        }
    }

    // 解析指令 (pub fn)
    let instruction_re = regex::Regex::new(r#"pub fn (\w+)\(ctx: Context<([^>]+)>(?:, ([^)]+))?\)"#).unwrap();
    for caps in instruction_re.captures_iter(content) {
        let name = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let _context = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
        let args_str = caps.get(3).map(|m| m.as_str().to_string()).unwrap_or_default();

        let mut arguments = Vec::new();
        if !args_str.is_empty() {
            for arg in args_str.split(',') {
                let arg = arg.trim();
                if let Some((name_part, type_part)) = arg.split_once(':') {
                    arguments.push(ArgumentInfo {
                        name: name_part.trim().to_string(),
                        type_name: type_part.trim().to_string(),
                    });
                }
            }
        }

        program_info.instructions.push(InstructionInfo {
            name,
            arguments,
        });
    }

    // 解析账户结构 (#[derive(Accounts)] pub struct)
    let account_re = regex::Regex::new(r#"#\[derive\(Accounts\)\]\s+pub struct (\w+)<[^>]*>\s*\{([^}]+)\}"#).unwrap();
    for caps in account_re.captures_iter(content) {
        let name = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let fields_str = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

        let mut fields = Vec::new();
        let mut current_field = String::new();

        for line in fields_str.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // 如果是属性行，继续累积
            if line.starts_with('#') {
                current_field.push_str(line);
                current_field.push(' ');
            } else {
                // 如果是字段定义行
                if let Some((name_part, type_part)) = line.split_once(':') {
                    let field_name = name_part.split_whitespace().last().unwrap_or("").to_string();
                    fields.push(FieldInfo {
                        name: field_name,
                        type_name: type_part.trim().to_string(),
                    });
                }
                current_field.clear();
            }
        }

        program_info.accounts.push(AccountInfo { name, fields });
    }

    // 解析错误 (pub enum Error)
    let error_re = regex::Regex::new(r#"pub enum Error\s*\{([^}]+)\}"#).unwrap();
    if let Some(caps) = error_re.captures(content) {
        let errors_str = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        for (idx, line) in errors_str.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let name = line.split_whitespace().next().unwrap_or("").to_string();
            program_info.errors.push(ErrorInfo {
                name: name.clone(),
                code: idx as u32 + 6000,
                message: name,
            });
        }
    }

    // 解析普通结构体 (pub struct)
    let struct_re = regex::Regex::new(r#"pub struct (\w+)\s*\{([^}]+)\}"#).unwrap();
    for caps in struct_re.captures_iter(content) {
        let name = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let fields_str = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

        let mut fields = Vec::new();
        for line in fields_str.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((name_part, type_part)) = line.split_once(':') {
                fields.push(FieldInfo {
                    name: name_part.trim().to_string(),
                    type_name: type_part.trim().to_string(),
                });
            }
        }

        program_info.structs.push(StructInfo { name, fields });
    }

    program_info
}