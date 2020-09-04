use std::collections::LinkedList;
use std::num::ParseIntError;

/* 每個欄位的名稱及其長度 */
pub struct Rec {
	name: &'static str,
	len : usize,
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
fn init_ord_rpt_format() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.append(&mut init_header());
	lst.push_back(Rec{ name: "RptSeq",   len: 8 });
	lst.push_back(Rec{ name: "BrkNo",    len: 7 });
	lst.push_back(Rec{ name: "IvacNo",   len: 7 });
	lst.push_back(Rec{ name: "DstBrkNo", len: 7 });
	lst.push_back(Rec{ name: "OrdNo",    len: 20 });
	lst.push_back(Rec{ name: "PreOrder", len: 1 });
	lst.push_back(Rec{ name: "FuncCmd",  len: 2 });
	lst.push_back(Rec{ name: "Symbol", len: 40 });
	lst.push_back(Rec{ name: "Side", len: 1 });
	lst.push_back(Rec{ name: "PriType", len: 1 });
	lst.push_back(Rec{ name: "PriSgn", len: 1 });
	lst.push_back(Rec{ name: "Pri99", len: 18 });
	lst.push_back(Rec{ name: "Qty", len: 8 });
	lst.push_back(Rec{ name: "SegMkt", len: 1 });
	lst.push_back(Rec{ name: "FuoPosEff", len: 1 });
	lst.push_back(Rec{ name: "TIF", len: 1 });
	lst.push_back(Rec{ name: "SalesNo", len: 8 });
	lst.push_back(Rec{ name: "IvacKind", len: 1 });
	lst.push_back(Rec{ name: "OrdYYYYMMDD", len: 8 });
	lst.push_back(Rec{ name: "OrdHHMMSS", len: 6 });
	lst.push_back(Rec{ name: "IDNO", len: 16 });
	lst.push_back(Rec{ name: "OrdKey", len: 64 });
	lst.push_back(Rec{ name: "PlatFlag", len: 1 });
	lst.push_back(Rec{ name: "SrcFlag", len: 1 });
	lst.push_back(Rec{ name: "OrgIvacKind", len: 1 });
	lst.push_back(Rec{ name: "OrgOrdYYYYMMDD", len: 14 });
	lst.push_back(Rec{ name: "OrgOrdKey", len: 64 });
	lst.push_back(Rec{ name: "OrgPlatFlag", len: 1 });
	lst.push_back(Rec{ name: "OrgSrcFlag", len: 1 });
	lst.push_back(Rec{ name: "OrgOrdQty", len: 8 });
	lst.push_back(Rec{ name: "LeavesQty", len: 8 });
	lst.push_back(Rec{ name: "TMatQty", len: 8 });
	lst.push_back(Rec{ name: "TDelQty", len: 8 });
	lst.push_back(Rec{ name: "UDelQty", len: 8 });
	lst.push_back(Rec{ name: "ErrCode", len: 4 });
	lst.push_back(Rec{ name: "ErrMsg", len: 80 });
	lst.push_back(Rec{ name: "ExchgMkt", len: 1 });
	lst.push_back(Rec{ name: "ExchgID", len: 10 });
	lst.push_back(Rec{ name: "Price2", len: 18 });
	lst.push_back(Rec{ name: "GTDateYYYYMMDD", len: 8 });
	lst.push_back(Rec{ name: "BasketID", len: 16 });
	lst
}

#[allow(dead_code)]
fn init_deal_rpt_format() -> LinkedList<Rec> {
	let mut lst = LinkedList::<Rec>::new();
	lst.append(&mut init_header());
    lst.push_back(Rec{ name: "RptSeq", len: 8 });
    lst.push_back(Rec{ name: "BrkNo", len: 7 });
    lst.push_back(Rec{ name: "IvacNo", len: 7 });
    lst.push_back(Rec{ name: "OrdNo", len: 20 });
    lst.push_back(Rec{ name: "CombFlag", len: 1 });
    lst.push_back(Rec{ name: "Symbol", len: 40 });
    lst.push_back(Rec{ name: "Symbol1", len: 20 });
    lst.push_back(Rec{ name: "Symbol2", len: 20 });
    lst.push_back(Rec{ name: "Side", len: 1 });
    lst.push_back(Rec{ name: "SideLeg1", len: 1 });
    lst.push_back(Rec{ name: "SideLeg2", len: 1 });
    lst.push_back(Rec{ name: "OrgOrdPriType", len: 1 });
    lst.push_back(Rec{ name: "OrgOrdSgnPri", len: 19 });
    lst.push_back(Rec{ name: "SegMkt", len: 1 });
    lst.push_back(Rec{ name: "FuoPosEff", len: 1 });
    lst.push_back(Rec{ name: "TIF", len: 1 });
    lst.push_back(Rec{ name: "SalesNo", len: 8 });
    lst.push_back(Rec{ name: "IvacKind", len: 1 });
    lst.push_back(Rec{ name: "OrdYMD", len: 8 });
    lst.push_back(Rec{ name: "OrdHMS", len: 6 });
    lst.push_back(Rec{ name: "IDNO", len: 16 });
    lst.push_back(Rec{ name: "OrgOrdKey", len:  64 });
    lst.push_back(Rec{ name: "OrgPlatFlag", len: 1 });
    lst.push_back(Rec{ name: "OrgSrcFlag", len: 1 });
    lst.push_back(Rec{ name: "OrgOrdQty", len: 8 });
    lst.push_back(Rec{ name: "LeavesQty", len: 8 });
    lst.push_back(Rec{ name: "TMatQty", len: 8 });
    lst.push_back(Rec{ name: "TDelQty", len: 8 });
    lst.push_back(Rec{ name: "MatYMD", len:  8 });
    lst.push_back(Rec{ name: "MatHMS", len:  6 });
    lst.push_back(Rec{ name: "MatQty", len:  8 });
    lst.push_back(Rec{ name: "MatSgnPri", len: 19 });
    lst.push_back(Rec{ name: "MatPri1", len: 18 });
    lst.push_back(Rec{ name: "MatPri2", len: 18 });
    lst.push_back(Rec{ name: "AvgSgnPri", len: 19 });
    lst.push_back(Rec{ name: "AvgPri1", len: 18 });
    lst.push_back(Rec{ name: "AvgPri2", len: 18 });
    lst.push_back(Rec{ name: "BrkRptSeq", len: 6 });
    lst.push_back(Rec{ name: "MktRptSeq", len: 8 });
    lst.push_back(Rec{ name: "ExchgMkt", len: 1 });
    lst.push_back(Rec{ name: "ExchgID", len: 10 });
    lst.push_back(Rec{ name: "Price2", len: 18 });
    lst.push_back(Rec{ name: "GTYMD", len: 8 });
    lst.push_back(Rec{ name: "BasketID", len: 16 });
	lst
}

#[allow(dead_code)]
fn struct_len(lst: &LinkedList<Rec>) -> usize {
	let mut sz : usize = 0;
	for field in lst {
		sz = sz + field.len;
	}
	sz
}

/* Parser本體宣告, 內含HTS Log欄位結構的LinkList */
pub struct Parser {
	hts_ord_rpt_format   : LinkedList<Rec>,
	hts_deal_rpt_format  : LinkedList<Rec>,
}

/* Parser方法實作 */
impl Parser {
	// 初始化
	pub fn new() -> Parser {
		Parser{ 
			hts_ord_rpt_format   : init_ord_rpt_format(),
			hts_deal_rpt_format  : init_deal_rpt_format(),
		}
	}
	// 從log line的標頭來檢查版本，以便使用符合的電文格式
	#[allow(dead_code)]
	pub fn check_version(&self, line: &str) -> Result<usize, ParseIntError> {
		if line.len() > 5 {
			Ok(line[1..5].parse::<usize>()?)
		} else {
			Ok(0)
		}
	}
	// 檢查為委託或成交格式
	pub fn check_format(&self, line: &str) -> Result<usize, ParseIntError> {
		if line.len() > 10 {
			Ok(line[5..9].parse::<usize>()?)
		} else {
			Ok(0)
		}
	}
	// 列出欄位 = 值
	pub fn show_list(&self, line: &str, fmtlist: &LinkedList<Rec>) {
		let mut beg_pos :usize = 0;
		let mut end_pos :usize = 0;
		let linelen = line.len();
		for recfmt in fmtlist {
			end_pos = end_pos + recfmt.len;
			if end_pos > linelen { break; }
			println!("{}={}", recfmt.name, &line[beg_pos..end_pos]);
			beg_pos = end_pos;
		}
	}
	// 輸入一行log
	pub fn parse_line(&self, line: &str) {
		println!("\n======Origin======\n{}:\n======Parsed======", line);
		if let Ok(format) = self.check_format(line) {
			match format {
				1011 => self.show_list(line, &self.hts_ord_rpt_format),
				1021 => self.show_list(line, &self.hts_deal_rpt_format),
				9001 => (),
				9002 => (),
				_ => println!("unknown format!"),
			}
		}
	}
	// 取得欄位名稱
	#[allow(dead_code)]
	pub fn get_field_names() -> &'static str {
		""
	}
}