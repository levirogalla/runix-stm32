use quote::quote;
use syn::{
    Attribute, DeriveInput, Meta, parse::Parse, parse_macro_input,
    punctuated::Punctuated,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(name);
    custom_keyword!(size);
}

#[cfg_attr(feature = "macro-debug", derive(Debug))]
enum RegArg {
    Name(RegName),
    Size(RegSize),
}

impl Parse for RegArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(kw::name) {
            Ok(RegArg::Name(input.parse::<RegName>()?))
        } else if input.peek(kw::size) {
            Ok(RegArg::Size(input.parse::<RegSize>()?))
        } else {
            Err(syn::Error::new(
                input.span(),
                "unexpected register attribute",
            ))
        }
    }
}

#[derive(Default)]
#[cfg_attr(feature = "macro-debug", derive(Debug))]
struct RegAttrs {
    name: Option<RegName>,
    size: Option<RegSize>,
}

impl Parse for RegAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<RegArg, syn::Token![,]>::parse_terminated(input)?;
        let mut reg_name: Option<RegName> = None;
        let mut reg_size: Option<RegSize> = None;
        for arg in args {
            match arg {
                RegArg::Name(name) => {
                    reg_name = Some(name);
                }
                RegArg::Size(size) => {
                    reg_size = Some(size);
                }
            }
        }
        Ok(Self {
            name: reg_name,
            size: reg_size,
        })
    }
}

#[cfg_attr(feature = "macro-debug", derive(Debug))]
struct RegName(syn::LitStr);

impl Parse for RegName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = input.parse::<kw::name>()?;
        _ = input.parse::<syn::Token![=]>()?;
        let name: syn::LitStr = input.parse()?;
        Ok(Self(name))
    }
}

#[cfg_attr(feature = "macro-debug", derive(Debug))]
struct RegSize(syn::Type);

impl Parse for RegSize {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = input.parse::<kw::size>()?;
        _ = input.parse::<syn::Token![=]>()?;
        let size = input.parse::<syn::Type>()?;
        Ok(Self(size))
    }
}

fn parse_attrs(attrs: Vec<Attribute>) -> Option<RegAttrs> {
    for attr in attrs {
        match attr.meta {
            // Meta::NameValue(val) => {
            //     // println!("{:?}", val);
            //     // Some(val.path.get_ident().unwrap().to_string())
            // }
            Meta::List(list) => {
                let args: RegAttrs = list.parse_args().unwrap();
                return Some(args);
            }
            _ => {
                panic!()
            }
        }
    }
    None
}

#[proc_macro_derive(Reg, attributes(reg))]
pub fn derive_reg(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = parse_attrs(input.attrs).unwrap_or_default();
    let reg_size: syn::Type = attrs.size.map(|s| s.0).unwrap_or_else(|| syn::parse_quote!(i32));
    quote! {
        impl crate::hardware::registers::Reg for #name {
            type Type = #reg_size;
        }
    }
    .into()
}

#[proc_macro_derive(SafeRead, attributes(reg))]
pub fn derive_safe_read_reg(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = parse_attrs(input.attrs).unwrap_or_default();
    let reg_name = attrs.name.map(|n| n.0.value()).unwrap_or_else(|| name.to_string());
    let asm_line = format!("mov {}, {}", "{}", reg_name.to_lowercase());
    quote! {
        impl crate::hardware::registers::SafeReadReg for #name {
            #[inline(always)]
            fn read() -> Self::Type {
                let reg_val: Self::Type;
                unsafe {
                    asm!(#asm_line, out(reg) reg_val);
                }
                reg_val
            }
        }
    }
    .into()
}

#[proc_macro_derive(UnsafeRead, attributes(reg))]
pub fn derive_unsafe_read_reg(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = parse_attrs(input.attrs).unwrap_or_default();
    let reg_name = attrs.name.map(|n| n.0.value()).unwrap_or_else(|| name.to_string());
    let asm_line = format!("mov {}, {}", "{}", reg_name.to_lowercase());
    quote! {
        impl crate::hardware::registers::UnsafeReadReg for #name {
            #[inline(always)]
            unsafe fn read_raw() -> Self::Type {
                let reg_val: Self::Type;
                asm!(#asm_line, out(reg) reg_val);
                reg_val
            }
        }
    }
    .into()
}

#[proc_macro_derive(SafeWrite, attributes(reg))]
pub fn derive_safe_write_reg(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = parse_attrs(input.attrs).unwrap_or_default();
    let reg_name = attrs.name.map(|n| n.0.value()).unwrap_or_else(|| name.to_string());
    let asm_line = format!("mov {}, {}", reg_name.to_lowercase(), "{}");
    quote! {
        impl crate::hardware::registers::SafeWriteReg for #name {
            #[inline(always)]
            fn write(val: Self::Type) {
                unsafe {
                    asm!(#asm_line, in(reg) val);
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(UnsafeWrite, attributes(reg))]
pub fn derive_unsafe_write_reg(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = parse_attrs(input.attrs).unwrap_or_default();
    let reg_name = attrs.name.map(|n| n.0.value()).unwrap_or_else(|| name.to_string());
    let asm_line = format!("mov {}, {}", reg_name.to_lowercase(), "{}");
    quote! {
        impl crate::hardware::registers::UnsafeWriteReg for #name {
            unsafe fn write_raw(val: Self::Type) {
                #[inline(always)]
                asm!(#asm_line, in(reg) val);
            }
        }
    }
    .into()
}

