# symbol

A symbol table implementation for Rust.

Unlike other libraries, it doesn't just use a global `'static` symbol table. It is lifetimed -- the symbol table is deleted when you're done with it, and symbols are scoped to the table -- and it supports multiple tables, each with its own symbol type.


## Using

Cargo.toml:

	[package]
	...

	[dependencies]
	symbol = { git = "https://github.com/andy-hanson/rust-symbol.git" }

src/main.rs:

	extern crate core; // Used by symbol_type macro
	#[macro_use(symbol_type)]
	extern crate symbol;
	use symbol::SymbolTable;

	// Declare a symbol type for this table.
	symbol_type!(TestSymbol);

	fn main() {
		let mut table: SymbolTable<TestSymbol> = SymbolTable::new();
		let sym1 = table.intern(&"foofoo"[0..2]);
		let sym2 = table.intern(&"foofoo"[3..5]);
		assert_eq!(sym1, sym2);
	}


## Details

The [source code](src/lib.rs) is fairly short.

The table does not copy string data to itself, so any strings referenced must have a lifetime greater than the table.

Every symbol type derives from the `Symbol` trait. `Symbol`s are `Copy` and should not be borrowed. They have a lifetime parameter to ensure that they don't last longer than the strings they reference.
