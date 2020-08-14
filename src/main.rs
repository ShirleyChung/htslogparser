
use structopt::StructOpt;
use std::io::*;
use std::io::{BufReader};
//use std::io::prelude::*;
use std::fs::File;

mod fileread;
use crate::fileread::*;

mod parser;
use crate::parser::*;

// 1.參數取得
#[derive(StructOpt)]
struct Options {
	/// 要解析的HtsT.log
	filepath: String, // Log檔路徑
	/// 指定查詢的欄位
	#[structopt(short="f", long="field", default_value = "")]
	field   : String,
	/// SorReqOrd.log 檔案編碼格式, 預設BIG5
	#[structopt(short="e", long="encoding", default_value = "BIG5")]
	encoding: String,
	/// 輸出存檔
	#[structopt(short="s", long="save")]
	save: bool,	
	/// 不印出搜尋結果list
	#[structopt(short="h", long="hide")]
	hide: bool,		
	/// 選擇存檔路徑
	#[structopt(short="o", long="output", default_value = "")]
	savepath: String,
}

fn main() -> Result<()> {
	let options    = Options::from_args();

	let f          = File::open(options.filepath)?;
	let mut reader = BufReader::new(f);
	let mut parser = Parser::new();
	
	// 依每行解析
	read_data_log(&mut reader, &mut parser, &options.encoding);
	Ok(())
}
