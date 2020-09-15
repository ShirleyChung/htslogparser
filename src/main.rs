
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
	/// 要解析的HtsT.log路徑
	filepath: String, 
	/// 搜尋目標: 欄位名稱:值
	#[structopt(short="f", long="findby", default_value = "")]
	target: String,
	/// SorReqOrd.log 檔案編碼格式, 預設BIG5
	#[structopt(short="e", long="encoding", default_value = "BIG5")]
	encoding: String,
	/// 將欄位空白值以 '_' 取代
	#[structopt(short="r", long="repl")]
	replace: bool,
}

fn main() -> Result<()> {
	let options    = Options::from_args();

	let f          = File::open(options.filepath)?;
	let mut reader = BufReader::new(f);
	let mut parser = Parser::new(options.replace);
	
	// 依每行解析
	read_data_log(&mut reader, &mut parser, &options.encoding);
	// 搜尋目標
	if !&options.target.is_empty() {
		parser.find_by(&options.target);
	}
	Ok(())
}
