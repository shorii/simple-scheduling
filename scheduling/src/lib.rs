use proc_macro::TokenStream;
use scheduling_internals;

#[proc_macro_attribute]
pub fn scheduling(attr: TokenStream, item: TokenStream) -> TokenStream {
    scheduling_internals::scheduling(attr.into(), item.into()).into()
}
