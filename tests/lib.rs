extern crate core;
#[macro_use(symbol_type)]
extern crate symbol;
use symbol::SymbolTable;

symbol_type!(TestSymbol);

#[test]
fn eq() {
	let s1 = TestSymbol("foo");
	let s2 = TestSymbol("bar");
	let s3 = s1;
	assert!(s1 != s2);
	assert_eq!(s1, s3);
}

#[test]
// Symbols can be used in a hash map.
fn hash() {
	use std::collections::{HashMap};
	let s1 = TestSymbol("foo");
	let s2 = TestSymbol("bar");
	
	let mut hash: HashMap<TestSymbol, i32> = HashMap::new();
	hash.insert(s1, 1);
	hash.insert(s2, 2);
	assert_eq!(hash.get(&s1), Some(&1));
	assert_eq!(hash.get(&s2), Some(&2));
}

symbol_type!(EqSymbol);

#[test]
fn equality() {
	let base = "foofoo";
	let str1: &str = &base[0..2];
	let str2: &str = &base[3..5];

	// They are equal due to slow comparison, but not equal as pointers.
	assert_eq!(str1, str2);
	assert!(str1 as *const _ != str2 as *const _);

	let mut table: SymbolTable<EqSymbol> = SymbolTable::new();

	let sym1 = table.intern(str1);
	let sym2 = table.intern(str2);

	// The symbols are equal and use fast comparison.
	assert_eq!(sym1, sym2);
	assert_eq!(sym1.0 as *const _ as *const (), sym2.0 as *const _ as *const ());

	let sym3 = table.intern(base);
	assert!(sym1 != sym3);
}