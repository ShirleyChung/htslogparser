use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt;
use chrono::prelude::*;

/* 每個欄位的名稱及其長度 */
pub struct Rec {
	name: String,
	len : i32,
}

/* Parser本體宣告, 內含HTS Log欄位結構的LinkList */
pub struct Parser {
	hts_v2_format : LinkedList<Rec>,
	
}

/* Parser方法實作 */
impl Parser {
	pub fn new() -> Parser {
		Parser{ 
			hts_v2_format : LinkedList::<Rec>::new(),
		}
	}
	pub fn parse_line(&self, line: &str) {
		for recfmt in &self.hts_v2_format {
			println!("{} has len {}", recfmt.name, recfmt.len);
		}
		println!("we have line: {}", line);
	}
	pub fn say_hello(&self) {
		self.parse_line("12345");
		println!("hello! from parser");
	}
	#[allow(dead_code)]
	pub fn get_field_names() -> String {
		"".to_string()
	}
}