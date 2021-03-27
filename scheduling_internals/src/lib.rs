#[use_macro]
extern crate lazy_static;

use proc_macro2::TokenStream;
use quote;
use syn;


pub struct ScheduleManager {
    completed: bool,
}

pub fn scheduling(attr: TokenStream, item: TokenStream) -> TokenStream {
    let timer_interval: syn::LitInt = syn::parse2(attr.clone()).unwrap();
    let item_fn: syn::ItemFn = syn::parse2(item.clone()).unwrap();
    let orig_ident = item_fn.sig.ident;
    let ident = quote::format_ident!("_{}", orig_ident);
    let block = item_fn.block;
    let new_item_fn = quote::quote! {
        fn #orig_ident() -> Result<(), Box<dyn std::error::Error>> {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                use async_stream::stream;
                use futures::future::{Fuse, FusedFuture, FutureExt};
                use futures::{pin_mut, select};
                use futures::stream::{StreamExt};
                use tokio::time::sleep;
                use std::time::Duration;
                let timer = stream! {
                    loop {
                        sleep(Duration::from_secs(#timer_interval)).await;
                        yield ();
                    }
                };

                let #ident = || async #block;
                let fuse_container = Fuse::terminated();
                pin_mut!(timer, fuse_container);

                loop {
                    select! {
                        () = timer.select_next_some() => {
                            if fuse_container.is_terminated() {
                                fuse_container.set(#ident().fuse());
                            }
                        },
                        () = fuse_container => {
                            continue;
                        },
                        complete => panic!("unexpectedly timer complete")
                    }
                }
            });
            Ok(())
        }
    };
    new_item_fn
}
