#[derive(Debug)]
pub struct AsmFile {
	pub name: String,
	pub content: Vec<AsmFileEntry>,
}

#[derive(Debug)]
pub enum AsmFileEntry {
	Pragma(String, MetaArg),
	Include(String),
	Value(Value),
	Relocate {
		name: String,
		addr: Expression,
	},
	Section {
		name: String,
		addr: Option<Expression>,
		metas: Vec<(String, MetaArg)>,
		content: Vec<SectionEntry>,
	},
}

#[derive(Debug)]
pub enum SectionEntry {
	Value(Value),
	Code {
		name: SymbolDecl,
		metas: Vec<(String, MetaArg)>,
		content: Vec<CodeEntry>,
	},
	Data {
		name: SymbolDecl,
		metas: Vec<(String, MetaArg)>,
		content: Vec<DataEntry>,
	},
}

#[derive(Debug)]
pub struct Value {
	pub name: SymbolDecl,
	pub content: Expression,
}

#[derive(Debug)]
pub enum CodeEntry {
	Label(SymbolDecl),
	Instruction {
		name: String,
		args: Vec<InstructionArg>,
	},
}

#[derive(Debug)]
pub enum DataEntry {
	Label(SymbolDecl),
	Reserve(ExprLeaf, Expression),
	Array(Vec<ExprLeaf>),
	String(String),
}

#[derive(Debug)]
pub enum InstructionArg {
	RegisterPrim(String),	// rp
	RegisterSecd(String),	// rs
	RegisterIp,				// ri
	RegisterJp,				// rj
	RegisterRf,				// rr
	RegisterSt,				// rt

	//AdjustIncrPrim(String),	// (aip)
	//AdjustDecrPrim(String),	// (adp)
	//AdjustPrimIncr(String),	// (api)
	//AdjustPrimDecr(String),	// (apd)
	AdjustIncrSect(String),	// ais
	AdjustDecrSect(String),	// ads
	AdjustSectIncr(String),	// asi
	//AdjustSectDecr(String),	// (asd)

	BitPrimImm(String, char),		// bpi
	BitPrimPrim(String, String),	// bpp
	//BitPrimSecd(String, String),	// (bps)
	//BitSecdImm(String, char),		// (bsi)
	//BitSecdPrim(String, String),	// (bsp)
	//BitSecdSecd(String, String),	// (bss)
	BitRfImm(char),					// bri
	BitRfPrim(String),				// brp
	//BitRfSecd(String),				// (brs)
	BitStImm(char),					// bti

	PropertyPrim(String, String),	// pp
	//PropertySecd(String, String),	// (ps)
	//PropertyRf(String, String),		// (pr)

	Condition(String),		// cnd
	String(String),			// str
	Expression(Expression),	// exp
}

/// Used Wherever ///

#[derive(Debug)]
pub enum Expression {
	Add(Box<Expression>, Box<Expression>),
	Sub(Box<Expression>, Box<Expression>),
	Mul(Box<Expression>, Box<Expression>),
	Leaf(ExprLeaf),
}

#[derive(Debug)]
pub enum ExprLeaf {
	Unsigned(u16),
	Signed(i16),
	Symbol(Symbol),
	SymbolSizeOf(Symbol),
	SymbolOffset(Symbol),
}

#[derive(Debug)]
pub enum Symbol {
	Section(String),
	Value(String),
	Block(String),
	Label(String),
	SectionValue(String, String),
	SectionBlock(String, String),
	BlockLabel(String, String),
}

#[derive(Debug)]
pub enum MetaArg {
	// Not final
	None,
	Unsigned(u16),
	SectionBlock(String, String),
}

#[derive(Debug)]
pub struct SymbolDecl {
	pub name: String,
	pub visible: bool,
}