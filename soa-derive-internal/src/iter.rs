use quote::{Tokens, Ident};
use syn;
use structs::Struct;

fn gen_zip_type(v: &[&syn::Ty]) -> Tokens {
    if v.len() >= 2 {
        let v0 = v[0];
        let v1 = v[1];
        return v.iter().skip(2).fold(quote!{iter::Zip<slice::Iter<'a, #v0>,slice::Iter<'a, #v1>>}, |q, &v| {quote!{iter::Zip<#q, slice::Iter<'a, #v>>}} );
    } else {
        let v0 = v[0];
        return quote!{(#v0)};
    };
}

fn gen_zip_tuple(v: &[syn::Ident]) -> Tokens {
    if v.len() >= 2 {
        let v0 = &v[0];
        let v1 = &v[1];
        return v.iter().skip(2).fold(quote!{(#v0,#v1)}, |q, ref v| {quote!{(#q, #v)}} );
    } else {
        let v0 = &v[0];
        return quote!{(#v0)};
    };
}

fn gen_zip_iter(v: &[syn::Ident]) -> Tokens {
    if ! v.is_empty() {
        let v0 = &v[0];
        return quote!{#v0.iter()#(.zip(#v.iter()))*};
    } else {
        return quote!{};
    }
}

pub fn derive(input: &Struct) -> Tokens {
    let name = &input.name;
    let visibility = &input.visibility;
    let detail_mod = Ident::from(format!("__detail_iter_{}", name.as_ref().to_lowercase()));
    let vec_name = &input.vec_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();

    let ref_doc_url = format!("[`{0}`](struct.{0}.html)", ref_name);
    let ref_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ref_mut_name);

    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;
    let first_field = &fields_names[0];

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    let zip_type = gen_zip_type(fields_types);
    let zip_tuple = gen_zip_tuple(&fields_names);
    let zip_iter = gen_zip_iter(&fields_names);

    let mut generated = quote! {
        #[allow(non_snake_case, dead_code)]
        mod #detail_mod {
            use super::*;
            use std::slice;

            #visibility struct Iter<'a> {
                #(pub(super) #fields_names_1: slice::Iter<'a, #fields_types>,)*
            }

            impl<'a> Iterator for Iter<'a> {
                type Item = #ref_name<'a>;
                fn next(&mut self) -> Option<#ref_name<'a>> {
                    #(let #fields_names_1 = self.#fields_names_2.next();)*
                    if #first_field.is_none() {
                        None
                    } else {
                        Some(#ref_name {
                            #(#fields_names_1: #fields_names_2.unwrap(),)*
                        })
                    }
                }
            }

            impl #vec_name {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this vector
                #visibility fn iter(&self) -> Iter {
                    Iter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }
            }

            impl<'a> #slice_name<'a> {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this slice.
                #visibility fn iter(&self) -> Iter {
                    Iter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }
            }

            #visibility struct IterMut<'a> {
                #(pub(super) #fields_names_1: slice::IterMut<'a, #fields_types>,)*
            }

            impl<'a> Iterator for IterMut<'a> {
                type Item = #ref_mut_name<'a>;
                fn next(&mut self) -> Option<#ref_mut_name<'a>> {
                    #(let #fields_names_1 = self.#fields_names_2.next();)*
                    if #first_field.is_none() {
                        None
                    } else {
                        Some(#ref_mut_name {
                            #(#fields_names_1: #fields_names_2.unwrap(),)*
                        })
                    }
                }
            }

            impl #vec_name {
                /// Get a mutable iterator over the
                #[doc = #ref_mut_doc_url]
                /// in this vector
                #visibility fn iter_mut(&mut self) -> IterMut {
                    IterMut {
                        #(#fields_names_1: self.#fields_names_2.iter_mut(),)*
                    }
                }
            }

            impl<'a> #slice_mut_name<'a> {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this vector
                #visibility fn iter(&mut self) -> Iter {
                    Iter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }

                /// Get a mutable iterator over the
                #[doc = #ref_mut_doc_url]
                /// in this vector
                #visibility fn iter_mut(&mut self) -> IterMut {
                    IterMut {
                        #(#fields_names_1: self.#fields_names_2.iter_mut(),)*
                    }
                }
            }
        }
    };

    if let syn::Visibility::Public = *visibility {
        generated.append(quote!{
            impl<'a> IntoIterator for #slice_name<'a> {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }
            }

            impl<'a,'b> IntoIterator for &'a #slice_name<'b> {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }
            }

            impl<'a> IntoIterator for &'a #vec_name {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }
            }

            impl<'a> IntoIterator for #slice_mut_name<'a> {
                type Item = #ref_mut_name<'a>;
                type IntoIter = #detail_mod::IterMut<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter {
                        #(#fields_names_1: self.#fields_names_2.iter_mut(),)*
                    }
                }
            }

            impl<'a> IntoIterator for &'a mut #vec_name {
                type Item = #ref_mut_name<'a>;
                type IntoIter = #detail_mod::IterMut<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter {
                        #(#fields_names_1: self.#fields_names_2.iter_mut(),)*
                    }
                }
            }
        });
    }

    return generated;
}
