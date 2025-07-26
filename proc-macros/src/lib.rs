// use proc_macro::*;
// use quote::*;
// use syn::*;

// example attribute macro
//#[proc_macro_attribute]
//fn example_attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
//    println!("attr: \"{attr}\"");
//    println!("item: \"{item}\"");
//    item
//}
//
// Example Derive Macro
//#[proc_macro_derive(AnswerFn)]
//fn derive_answer_fn(_item: TokenStream) -> TokenStream {
//    "fn answer() -> u32 { 42 }".parse().unwrap()
//}