pub use quote2::utils::quote_rep;

#[macro_export]
macro_rules! quote_into {
    ($($tts: tt)*) => {{
        let mut __o = TokenStream::new();
        quote2::quote!(__o, {
            $($tts)*
        });
        __o
    }};
}

#[macro_export]
macro_rules! quote_fn {
    (type $lt: lifetime) => { quote2::Token<impl Fn(&mut TokenStream) + $lt> };
    (type) => { quote2::Token<impl Fn(&mut TokenStream)> };
    ($span:tt=> $($t:tt)*) => {
        quote2::quote(move |t| {
            quote2::quote_spanned!($span, t, {
                $($t)*
            });
        })
    };
    ($($t:tt)*) => {
        quote2::quote(move |t| {
            quote2::quote!(t, {
                $($t)*
            });
        })
    };
}
