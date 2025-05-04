use anyhow::Result;
use clap::Parser;
use std::env;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize}; // bincode用にSerialize/Deserializeが必要

// ====================
// タグとして保存するデータ構造体
// ====================
#[derive(Serialize, Deserialize, Debug)]
struct TagData {
    config_file: PathBuf,
    program: PathBuf,
    args: Vec<String>,
}

// ====================
// メイン引数格納用構造体
// ====================
#[derive(Parser, Clone)]
struct Cli {
    /// タグ名（ファイル名として使用される）
    #[arg(long, short)]
    tag_name: String,

    #[arg(long)]
    config_file: PathBuf,
    /// 実行対象のプログラムパス
    #[arg(long)]
    program: PathBuf,

    /// 実行時の引数（スペース区切りで渡す）
    #[arg()]
    program_args: Vec<String>,
}

// ====================
// メイン関数
// ====================
fn main() -> Result<()> {
    // 引数をパース
    let cli = Cli::parse();

    // %USERPROFILE%\\.eec ディレクトリを取得
    let home_dir = env::var("USERPROFILE")?;
    let eec_dir = Path::new(&home_dir).join(".eec");
    create_dir_all(&eec_dir)?; // ディレクトリがなければ作成

    // タグファイルのパスを決定
    let tag_path = eec_dir.join(format!("{}.tag", cli.tag_name));

    // タグデータを構造体に格納
    let tag_data = TagData {
        config_file: cli.config_file,
        program: cli.program,
        args: cli.program_args,
    };

    // バイナリとしてファイルに保存
    let encoded = bincode::serialize(&tag_data)?;
    let mut file = File::create(&tag_path)?;
    file.write_all(&encoded)?;

    println!("Tag saved to {:?}", tag_path);
    Ok(())
}
