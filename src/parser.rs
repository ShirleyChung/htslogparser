use std::collections::LinkedList;

/* 每個欄位的名稱及其長度 */
pub struct Rec {
	name: &'static str,
	len : i32,
}

#[allow(dead_code)]
fn init_v2_ord_format() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.push_back(Rec{ name: "ESC",      len: 1 });
	lst.push_back(Rec{ name: "Ver",      len: 4 });
	lst.push_back(Rec{ name: "Fmt",      len: 4 });	
	lst
}

#[allow(dead_code)]
fn init_v2_rpt_format() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.push_back(Rec{ name: "ESC",      len: 1 });
	lst.push_back(Rec{ name: "Ver",      len: 4 });
	lst.push_back(Rec{ name: "Fmt",      len: 4 });
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
fn init_v2_deal_format() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.push_back(Rec{ name: "ESC",      len: 1 });
	lst.push_back(Rec{ name: "Ver",      len: 4 });
	lst.push_back(Rec{ name: "Fmt",      len: 4 });	
	lst
}

/* Parser本體宣告, 內含HTS Log欄位結構的LinkList */
pub struct Parser {
	hts_v2_format : LinkedList<Rec>,
	
}

/* Parser方法實作 */
impl Parser {
	pub fn new() -> Parser {
		Parser{ 
			hts_v2_format : init_v2_ord_format(),
		}
	}
	pub fn parse_line(&self, line: &str) {
		for recfmt in &self.hts_v2_format {
			println!("{} has len {}", recfmt.name, recfmt.len);
		}
		println!("we have line: {}", line);
	}
	#[allow(dead_code)]
	pub fn get_field_names() -> &'static str {
		""
	}
}