#![recursion_limit = "256"]

extern crate proc_macro;
use quote::*;
use syn::*;

use proc_macro::TokenStream;
use quote::TokenStreamExt;
use syn::DeriveInput;

fn new_ident(x: &str) -> syn::Ident {
    syn::Ident::new(x, proc_macro2::Span::call_site()).into()
}

fn get_str_inside_bracket(
    level: i8,
    lines: Vec<syn::Attribute>,
) -> Vec<(String, Vec<String>, Vec<String>)> {
    use syn::punctuated::Punctuated;
    use syn::Meta::{List, Word};
    use syn::NestedMeta;
    use syn::{Lit, Meta, MetaNameValue};

    lines
        .iter()
        // get one line meta
        .map(|a| {
            (
                a.path.segments[0].ident.to_string(),
                a.parse_meta().unwrap(),
            )
        })
        // just simple name="value", change the keyword
        .map(|(ident, a)| {
            if level == 0i8 {
                (
                    match a.clone() {
                        Meta::NameValue(MetaNameValue {
                            ident: id,
                            lit: Lit::Str(lit_str),
                            ..
                        }) => format!("{}==>>{}", id.to_string(), lit_str.value()),
                        _ => ident.clone(),
                    },
                    a,
                )
            } else {
                (ident, a)
            }
        })
        // get nested meta inside brackets
        .map(|(ident, a)| {
            let va = match a {
                List(l) => l.nested.clone(),
                _ => Punctuated::new(),
            };
            (ident, va)
        })
        // get keyword with class
        .map(|(ident, a)| {
            let va = a
                .into_iter()
                .map(|i| {
                    let keyword = match i.clone() {
                        // 0->ident 1->namevalue
                        NestedMeta::Meta(Word(i)) => (i.to_string(), 0u8),
                        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                            ident: id,
                            lit: Lit::Str(lit_str),
                            ..
                        })) => (format!("{}==>>{}", id.to_string(), lit_str.value()), 1u8),
                        _ => ("".to_string(), 0u8),
                    };
                    keyword
                })
                // .inspect(|i| println!("{:?}", i))
                .collect::<Vec<_>>();
            (ident, va)
        })
        // get tuple
        .map(|(ident, a)| {
            (
                ident,
                a.clone()
                    .into_iter()
                    .filter(|i| i.1 == 0u8)
                    .map(|i| i.0)
                    .collect::<Vec<_>>(),
                a.clone()
                    .into_iter()
                    .filter(|i| i.1 == 1u8)
                    .map(|i| i.0)
                    .collect::<Vec<_>>(),
            )
        })
        // .inspect(|i| println!("{:?}", i))
        // keyword([idents], [name<==>value])
        .collect::<Vec<(String, Vec<String>, Vec<String>)>>()
}

#[proc_macro_derive(
    RestrictionForDiesel,
    attributes(
        has_many_children,
        belongs_to_tables,
        brothers_impl_belongs,
        brothers_impl,
    )
)]
pub fn has_many_macro_derive(input: TokenStream) -> TokenStream {
    // let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let token0: proc_macro2::TokenStream =
        before_require(&ast).unwrap_or(TokenStream::new()).into();
    let token1: proc_macro2::TokenStream =
        impl_table_name(&ast).unwrap_or(TokenStream::new()).into();
    let token2: proc_macro2::TokenStream = impl_has_many_children_macro(&ast)
        .unwrap_or(TokenStream::new())
        .into();
    let token3: proc_macro2::TokenStream = impl_belongs_to_macro(&ast)
        .unwrap_or(TokenStream::new())
        .into();
    let token4: proc_macro2::TokenStream = impl_brothers_impl_macro(&ast)
        .unwrap_or(TokenStream::new())
        .into();
    let mut token = proc_macro2::TokenStream::new();
    token.append_all(token0);
    token.append_all(token1);
    token.append_all(token2);
    token.append_all(token3);
    token.append_all(token4);
    token.into()
}

fn before_require(_ast: &DeriveInput) -> Result<TokenStream, &str> {
    let gen = quote! {
        extern crate diesel;
        use ::diesel::query_dsl::*;
        use ::diesel::result::Error;
        use ::diesel::dsl::*;
    };
    Ok(gen.into())
}

fn impl_table_name(ast: &DeriveInput) -> Result<TokenStream, &str> {
    // get Derive basic name
    let name = &ast.ident;
    let attrs = get_str_inside_bracket(0, ast.attrs.clone());
    let attr = attrs
        .into_iter()
        .find(|a| a.0.starts_with("table_name==>>"))
        .ok_or("no attribute: table_name")?;
    let table_name = new_ident(attr.0.split("==>>").nth(1).unwrap());
    // println!("{:?}", table_name);
    // gen code
    let gen = quote! {
        impl #name {
            pub fn table_name() -> Result<String, Error> {
                Ok(stringify!{#table_name}.to_string())
            }
            pub fn delete(&self, conn: &diesel::pg::PgConnection) -> Result<usize, Error> {
                diesel::delete(self).execute(conn)
            }
        }
    };
    Ok(gen.into())
}

fn impl_has_many_children_macro(ast: &DeriveInput) -> Result<TokenStream, &str> {
    // get Derive basic name and attribute
    let name = ast.ident.clone();
    let attrs = get_str_inside_bracket(1, ast.attrs.clone());
    let attr = attrs
        .into_iter()
        .find(|a| a.0 == "has_many_children")
        .ok_or("no attribute: has_many_children")?;
    let token_vec = attr
        .1
        .clone()
        .into_iter()
        .map(|x| new_ident(x.as_str()))
        .collect::<Vec<_>>();
    let token_vec_clone = token_vec.clone();
    // gen code
    let gen = quote! {

        impl #name {
            pub fn has_any_child(&self, conn: &diesel::pg::PgConnection) -> Result<bool, Error> {
                #(
                    let res = #token_vec::belonging_to(self).limit(1)
                        .load::<#token_vec_clone>(conn)
                        .and_then(|n|
                            if n.len() != 0 { Ok(true) } else { Ok(false) }
                        )?;
                    // early return
                    if res { return Ok(true) };
                );*;
                Ok(false)
            }

            pub fn restrict_delete(&self, conn: &diesel::pg::PgConnection) -> Result<usize, Error> {
                self.has_any_child(conn)
                    .and_then(|b|
                        match b {
                            true  => Ok(0),
                            false => diesel::delete(self).execute(conn),
                        }
                    )
            }
        }
    };
    Ok(gen.into())
}

fn impl_belongs_to_macro(ast: &DeriveInput) -> Result<TokenStream, &str> {
    // get attribute
    let attrs = get_str_inside_bracket(0, ast.attrs.clone());
    let attr = attrs
        .clone()
        .into_iter()
        .find(|a| a.0.starts_with("table_name==>>"))
        .ok_or("no attribute: table_name")?;
    let table_name = new_ident(attr.0.split("==>>").nth(1).unwrap());
    // println!("{:?}", table_name);
    let token_vec_parents_table = attrs
        .clone()
        .into_iter()
        .find(|a| a.0.starts_with("belongs_to_tables==>>"))
        .ok_or("no attribute: belongs_to_tables")?;
    let token_vec_parents_table = token_vec_parents_table
        .0
        .split("==>>")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| new_ident(x.trim()))
        .collect::<Vec<_>>();
    // println!("{:?}", token_vec_parents_table);

    let attrs = get_str_inside_bracket(1, ast.attrs.clone());

    let lines = attrs
        .clone()
        .into_iter()
        .filter(|a| a.0 == "belongs_to")
        .collect::<Vec<_>>();
    if lines.len() == 0 {
        return Err("no attribute: belongs_to");
    };
    // println!("{:?}", lines);
    let token_vec_struct = lines
        .clone()
        .into_iter()
        .map(|x| new_ident(x.1[0].as_str()))
        .collect::<Vec<_>>();
    let token_vec_field = lines
        .clone()
        .into_iter()
        .map(|x| new_ident(x.2[0].split("foreign_key==>>").nth(1).unwrap()))
        .collect::<Vec<_>>();
    // println!("{:?}", token_vec_struct);

    let brothers = attrs
        .clone()
        .into_iter()
        .filter(|a| a.0 == "brothers_impl_belongs")
        .collect::<Vec<_>>();
    if brothers.len() == 0 {
        return Err("no attribute: brothers_impl_belongs");
    };
    // println!("{:?}", brothers);
    let token_vec_brothers = brothers
        .clone()
        .into_iter()
        .map(|x| {
            (
                new_ident(x.1[0].as_str()),
                x.2[0].split("brother_type==>>").nth(1).unwrap().to_string(),
                token_vec_parents_table.clone(),
            )
        })
        .collect::<Vec<_>>();
    // println!("brothers: {:?}", token_vec_brothers);

    // gen code
    let token = token_vec_brothers.into_iter()
        .fold(proc_macro2::TokenStream::new(), |mut sum, (brother, brother_type, token_vec_parents_table)| {
        let struct_name = brother.clone();
        let tables = token_vec_parents_table.clone();
        let structs = token_vec_struct.clone();
        let fields = token_vec_field.clone();
        let gen_1 = quote! {
            impl #struct_name {
                pub fn has_all_parents(&self, conn: &diesel::pg::PgConnection) -> Result<bool, Error> {
                    #(
                        // println!("{}", stringify!(#tables));
                        // println!("{}", stringify!(#structs));
                        // println!("{}", stringify!(#fields));
                        let vec_parents = #tables::table.find(self.#fields)
                            .limit(1)
                            .load::<#structs>(conn)?;
                        if vec_parents.len() == 0 { return Ok(false) };
                        // println!("{:?}", vec_parents);
                    )*;
                    Ok(true)
                }
            }
        };
        let gen_2 = if brother_type == "insert" {
            quote! {
                impl #struct_name {
                    pub fn restrict_insert(&self, conn: &diesel::pg::PgConnection) -> Result<usize, Error> {
                        self.has_all_parents(conn)
                            .and_then(|b|
                                match b {
                                    true  => diesel::insert_into(#table_name::table).values(self).execute(conn),
                                    false => Ok(0),
                                }
                            )
                    }
                }
            }
        } else { quote!() };
        let gen_3 = if brother_type == "update" {
            quote! {
                impl #struct_name {
                    pub fn restrict_update(&self, conn: &diesel::pg::PgConnection) -> Result<usize, Error> {
                        self.has_all_parents(conn)
                            .and_then(|b|
                                match b {
                                    true  => diesel::update(#table_name::table.find(self.id)).set(self).execute(conn),
                                    false => Ok(0),
                                }
                            )
                    }
                }
            }
        } else { quote!() };
        sum.append_all(gen_1);
        sum.append_all(gen_2);
        sum.append_all(gen_3);
        sum
    });

    Ok(token.into())
}

fn impl_brothers_impl_macro(ast: &DeriveInput) -> Result<TokenStream, &str> {
    let attrs = get_str_inside_bracket(0, ast.attrs.clone());
    let attr = attrs
        .clone()
        .into_iter()
        .find(|a| a.0.starts_with("table_name==>>"))
        .ok_or("no attribute: table_name")?;
    let table_name = new_ident(attr.0.split("==>>").nth(1).unwrap());
    // println!("{:?}", table_name);

    let brothers = attrs
        .clone()
        .into_iter()
        .filter(|a| a.0 == "brothers_impl")
        .collect::<Vec<_>>();
    if brothers.len() == 0 {
        return Err("no attribute: brothers_impl_belongs");
    };
    // println!("{:?}", brothers);
    let token_vec_brothers = brothers
        .clone()
        .into_iter()
        .map(|x| {
            (
                new_ident(x.1[0].as_str()),
                x.2[0].split("brother_type==>>").nth(1).unwrap().to_string(),
            )
        })
        .collect::<Vec<_>>();

    // gen code
    let token = token_vec_brothers.into_iter()
        .fold(proc_macro2::TokenStream::new(), |mut sum, (brother, brother_type)| {
        let struct_name = brother.clone();
        let gen_2 = if brother_type == "insert" {
            quote! {
                impl #struct_name {
                    pub fn insert(&self, conn: &diesel::pg::PgConnection) -> Result<usize, Error> {
                        diesel::insert_into(#table_name::table).values(self).execute(conn)
                    }
                }
            }
        } else { quote!() };
        let gen_3 = if brother_type == "update" {
            quote! {
                impl #struct_name {
                    pub fn update(&self, conn: &diesel::pg::PgConnection) -> Result<usize, Error> {
                        diesel::update(#table_name::table.find(self.id)).set(self).execute(conn)
                    }
                }
            }
        } else { quote!() };
        sum.append_all(gen_2);
        sum.append_all(gen_3);
        sum
    });

    Ok(token.into())
}
