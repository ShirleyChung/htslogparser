use std::collections::LinkedList;

/* 每個欄位的名稱及其長度 */
pub struct Rec {
	name: &'static str,
	len : u32,
}

struct FieldPos {
	begin: u32,
	end  : u32,
}

#[allow(dead_code)]
fn init_header() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.push_back(Rec{ name: "ESC",      len: 1 });
	lst.push_back(Rec{ name: "Ver",      len: 4 });
	lst.push_back(Rec{ name: "Fmt",      len: 4 });	
	lst
}

#[allow(dead_code)]
fn init_v2_ord_format() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.append(&mut init_header());
	lst
}

fn get_field_pos_vec(lst: &LinkedList<Rec>) -> Vec<FieldPos> {
	let mut vec = Vec::<FieldPos>::new();
	let mut cur_begin : u32 = 0;
	let mut cur_end   : u32 = 0;
	for item in lst {
		cur_begin = cur_end;
		cur_end = cur_begin + item.len;
		vec.push( FieldPos { begin: cur_begin, end: cur_end } );
	}
	vec
}

#[allow(dead_code)]
fn init_v2_rpt_format() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.append(&mut init_header());
	lst.push_back(Rec{ name: "RptSeq",   len: 8 });
	lst.push_back(Rec{ name: "BrkNo",    len: 7 });
	lst.push_back(Rec{ name: "IvacNo",   len: 7 });
	lst.push_back(Rec{ name: "DstBrkNo", len: 7 });
	lst.push_back(Rec{ name: "OrdNo",    len: 20 });
	lst.push_back(Rec{ name: "PreOrder", len: 1 });
	lst.push_back(Rec{ name: "FuncCmd",  len: 1 });
	lst.push_back(Rec{ name: "Symbol", len: 1 });
	lst.push_back(Rec{ name: "Side", len: 1 });
	lst.push_back(Rec{ name: "PriType", len: 1 });
	lst.push_back(Rec{ name: "PriSgn", len: 1 });
	lst.push_back(Rec{ name: "Pri99", len: 1 });
	lst.push_back(Rec{ name: "Qty", len: 1 });
	lst.push_back(Rec{ name: "SegMkt", len: 1 });
	lst.push_back(Rec{ name: "FuoPosEff", len: 1 });
	lst.push_back(Rec{ name: "TIF", len: 1 });
	lst.push_back(Rec{ name: "SalesNo", len: 1 });
	lst.push_back(Rec{ name: "IvacKind", len: 1 });
	lst.push_back(Rec{ name: "OrdYYYYMMDD", len: 1 });
	lst.push_back(Rec{ name: "OrdHHMMSS", len: 1 });
	lst.push_back(Rec{ name: "IDNO", len: 1 });
	lst.push_back(Rec{ name: "OrdKey", len: 1 });
	lst.push_back(Rec{ name: "PlatFlag", len: 1 });
	lst.push_back(Rec{ name: "SrcFlag", len: 1 });
	lst.push_back(Rec{ name: "OrgIvacKind", len: 1 });
	lst.push_back(Rec{ name: "OrgOrdYYYYMMDD", len: 1 });
	lst.push_back(Rec{ name: "OrgOrdKey", len: 1 });
	lst.push_back(Rec{ name: "OrgPlatFlag", len: 1 });
	lst.push_back(Rec{ name: "OrgSrcFlag", len: 1 });
	lst.push_back(Rec{ name: "OrgOrdQty", len: 1 });
	lst.push_back(Rec{ name: "LeavesQty", len: 1 });
	lst.push_back(Rec{ name: "TMatQty", len: 1 });
	lst.push_back(Rec{ name: "TDelQty", len: 1 });
	lst.push_back(Rec{ name: "UDelQty", len: 1 });
	lst.push_back(Rec{ name: "ErrCode", len: 1 });
	lst.push_back(Rec{ name: "ErrMsg", len: 1 });
	lst.push_back(Rec{ name: "ExchgMkt", len: 1 });
	lst.push_back(Rec{ name: "ExchgID", len: 1 });
	lst.push_back(Rec{ name: "Price2", len: 1 });
	lst.push_back(Rec{ name: "GTDateYYYYMMDD", len: 1 });
	lst.push_back(Rec{ name: "BasketID", len: 1 });
	lst.push_back(Rec{ name: "TERMINAL", len: 1 });
	lst
}

#[allow(dead_code)]
fn struct_len(lst: &LinkedList<Rec>) -> u32 {
	let mut sz : u32 = 0;
	for field in lst {
		sz = sz + field.len;
	}
	sz
}

#[allow(dead_code)]
fn init_v2_deal_format() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.append(&mut init_header());
	lst
}

/* Parser本體宣告, 內含HTS Log欄位結構的LinkList */
pub struct Parser {
	hts_v2_format     : LinkedList<Rec>,
	hts_v2_format_pos : Vec<FieldPos>,
}

/* Parser方法實作 */
impl Parser {
	// 初始化
	pub fn new() -> Parser {
		Parser{ 
			hts_v2_format     : init_v2_ord_format(),
			hts_v2_format_pos : get_field_pos_vec(&mut init_v2_ord_format()),
		}
	}
	// 從log line的標頭來檢查版本，以便使用符合的電文格式
	pub fn check_version(&self, line: &str) {
		if line.len() > 5 {
			println!("version is {}", &line[1..5]);
		}
	}
	// 輸入一行log
	pub fn parse_line(&self, line: &str) {
		for recfmt in &self.hts_v2_format {
			println!("{} has len {}", recfmt.name, recfmt.len);
		}
		println!("we have line: {}", line);
		self.check_version(line);
	}
	// 取得欄位名稱
	#[allow(dead_code)]
	pub fn get_field_names() -> &'static str {
		""
	}
}