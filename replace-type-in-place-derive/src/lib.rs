use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, TypeParam};

#[proc_macro_derive(Replace)]
pub fn derive_replace(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let replace_impls = generate_replace_impls(name, &input.data, generics);
    let expanded = quote! {
        #replace_impls
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(ReplaceInPlace)]
pub fn derive_replace_in_place(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let replace_in_place_impls = generate_replace_in_place_impls(name, &input.data, generics);
    let expanded = quote! {
        #replace_in_place_impls
    };
    TokenStream::from(expanded)
}

fn generate_replace_impls(
    name: &syn::Ident,
    data: &Data,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let type_params = generics.type_params().collect::<Vec<_>>();
    let type_param_names: Vec<_> = type_params.iter().map(|tp| &tp.ident).collect();

    let replace_methods = (1..=8).map(|i| {
        let method_name = format_ident!("replace_{}", i);
        let generic_params: Vec<_> = (0..i).map(|j| format_ident!("A{}", j)).collect();
        let fn_params: Vec<_> = type_param_names
            .iter()
            .take(i)
            .enumerate()
            .map(|(j, &tp)| {
                let fn_name = format_ident!("f{}", j);
                quote! { #fn_name: &mut impl FnMut(Self::#tp) -> #(#generic_params)::* }
            })
            .collect();
        let output_params: Vec<_> = type_param_names
            .iter()
            .enumerate()
            .map(|(j, &tp)| {
                if j < i {
                    quote! { #(#generic_params)::* }
                } else {
                    quote! { Self::#tp }
                }
            })
            .collect();
        let replace_fields = generate_replace_fields(name, data, &type_param_names, i);

        quote! {
            fn #method_name<#(#generic_params),*>(
                self,
                #(#fn_params),*
            ) -> Self::OutputSelf<#(#output_params),*> {
                #replace_fields
            }
        }
    });

    quote! {
        impl #impl_generics replace_type_in_place::Replace for #name #ty_generics #where_clause {
            type AOld = #(#type_param_names)::*;
            type BOld = ();
            type COld = ();
            type DOld = ();
            type EOld = ();
            type FOld = ();
            type GOld = ();
            type HOld = ();
            type OutputSelf<AN, BN, CN, DN, EN, FN, GN, HN> = #name<AN, BN, CN, DN, EN, FN, GN, HN>;

            #(#replace_methods)*
        }
    }
}

fn generate_replace_in_place_impls(
    name: &syn::Ident,
    data: &Data,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let type_params = generics.type_params().collect::<Vec<_>>();
    let type_param_names: Vec<_> = type_params.iter().map(|tp| &tp.ident).collect();

    let replace_in_place_methods = (1..=8).map(|i| {
        let method_name = format_ident!("replace_in_place_{}", i);
        let generic_params: Vec<_> = (0..i).map(|j| format_ident!("A{}", j)).collect();
        let fn_params: Vec<_> = type_param_names
            .iter()
            .take(i)
            .enumerate()
            .map(|(j, &tp)| {
                let fn_name = format_ident!("f{}", j);
                quote! { #fn_name: &mut impl FnMut(Self::#tp) -> #(#generic_params)::* }
            })
            .collect();
        let output_params: Vec<_> = type_param_names
            .iter()
            .enumerate()
            .map(|(j, &tp)| {
                if j < i {
                    quote! { #(#generic_params)::* }
                } else {
                    quote! { Self::#tp }
                }
            })
            .collect();
        let replace_in_place_fields =
            generate_replace_in_place_fields(name, data, &type_param_names, i);

        quote! {
            fn #method_name<#(#generic_params),*>(
                self,
                #(#fn_params),*
            ) -> Self::OutputSelf<#(#output_params),*> {
                #replace_in_place_fields
            }
        }
    });

    quote! {
        impl #impl_generics replace_type_in_place::ReplaceInPlace for #name #ty_generics #where_clause {
            type AOld = #(#type_param_names)::*;
            type BOld = ();
            type COld = ();
            type DOld = ();
            type EOld = ();
            type FOld = ();
            type GOld = ();
            type HOld = ();
            type OutputSelf<AN, BN, CN, DN, EN, FN, GN, HN> = #name<AN, BN, CN, DN, EN, FN, GN, HN>;

            #(#replace_in_place_methods)*
        }
    }
}

fn generate_replace_fields(
    name: &syn::Ident,
    data: &Data,
    param_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_replacements = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    if type_is_param(field_type, param_name) {
                        quote! { #field_name: f(self.#field_name), }
                    } else if type_contains_param(field_type, param_name) {
                        quote! { #field_name: <#field_type as replace_type_in_place::Replace<#param_name>>::replace(self.#field_name, f), }
                    } else {
                        quote! { #field_name: self.#field_name, }
                    }
                });
                quote! { #name { #(#field_replacements)* } }
            }
            Fields::Unnamed(ref fields) => {
                let field_replacements = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = syn::Index::from(i);
                    let field_type = &f.ty;
                    if type_is_param(field_type, param_name) {
                        quote! { f(self.#index), }
                    } else if type_contains_param(field_type, param_name) {
                        quote! { <#field_type as replace_type_in_place::Replace<#param_name>>::replace(self.#index, f), }
                    } else {
                        quote! { self.#index, }
                    }
                });
                quote! { #name(#(#field_replacements)*) }
            }
            Fields::Unit => quote! { #name },
        },
        Data::Enum(ref data) => {
            let variant_replacements = data.variants.iter().map(|v| {
                let variant_name = &v.ident;
                match &v.fields {
                    Fields::Named(ref fields) => {
                        let field_replacements = fields.named.iter().map(|f| {
                            let field_name = &f.ident;
                            let field_type = &f.ty;
                            if type_is_param(field_type, param_name) {
                                quote! { #field_name: f(#field_name), }
                            } else if type_contains_param(field_type, param_name) {
                                quote! { #field_name: <#field_type as replace_type_in_place::Replace<#param_name>>::replace(#field_name, f), }
                            } else {
                                quote! { #field_name, }
                            }
                        });
                        let field_patterns = fields.named.iter().map(|f| {
                            let field_name = &f.ident;
                            quote! { #field_name }
                        });
                        quote! {
                            #name::#variant_name { #(#field_patterns),* } => #name::#variant_name { #(#field_replacements)* },
                        }
                    },
                    Fields::Unnamed(ref fields) => {
                        let field_names: Vec<syn::Ident> = (0..fields.unnamed.len())
                            .map(|i| format_ident!("field{}", i))
                            .collect();
                        let field_replacements = fields.unnamed.iter().zip(field_names.iter()).map(|(f, field_name)| {
                            let field_type = &f.ty;
                            if type_is_param(field_type, param_name) {
                                quote! { f(#field_name), }
                            } else if type_contains_param(field_type, param_name) {
                                quote! { <#field_type as replace_type_in_place::Replace<#param_name>>::replace(#field_name, f), }
                            } else {
                                quote! { #field_name, }
                            }
                        });
                        quote! {
                            #name::#variant_name(#(#field_names),*) => #name::#variant_name(#(#field_replacements)*),
                        }
                    },
                    Fields::Unit => quote! {
                        #name::#variant_name => #name::#variant_name,
                    },
                }
            });
            quote! {
                match self {
                    #(#variant_replacements)*
                }
            }
        }
        Data::Union(_) => unimplemented!("Unions are not supported"),
    }
}

fn type_is_param(ty: &syn::Type, params: &[&syn::Ident]) -> bool {
    match ty {
        syn::Type::Path(type_path) if type_path.path.segments.len() == 1 => {
            params.contains(&&type_path.path.segments[0].ident)
        }
        _ => false,
    }
}

fn type_contains_param(ty: &syn::Type, params: &[&syn::Ident]) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path.path.segments.iter().any(|segment| {
            params.contains(&&segment.ident)
                || match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => args.args.iter().any(|arg| {
                        if let syn::GenericArgument::Type(t) = arg {
                            type_contains_param(t, params)
                        } else {
                            false
                        }
                    }),
                    _ => false,
                }
        }),
        _ => false,
    }
}

fn generate_replace_in_place_fields(
    name: &syn::Ident,
    data: &Data,
    type_params: &[&syn::Ident],
    num_params: usize,
) -> proc_macro2::TokenStream {
    let checks = (0..num_params).map(|i| {
        let param = type_params[i];
        let new_type = format_ident!("A{}", i);
        quote! {
            if std::mem::size_of::<#param>() != std::mem::size_of::<#new_type>() {
                panic!(
                    "The Old type has a different size than the New type you tried to replace it with: \n\
                    Old: {} size: {}\n\
                    New: {} size: {}",
                    std::any::type_name::<#param>(),
                    std::mem::size_of::<#param>(),
                    std::any::type_name::<#new_type>(),
                    std::mem::size_of::<#new_type>()
                );
            }

            if std::mem::align_of::<#param>() != std::mem::align_of::<#new_type>() {
                panic!(
                    "The Old type has a different alignment than the New type you tried to replace it with: \n\
                    Old: {} alignment: {}\n\
                    New: {} alignment: {}",
                    std::any::type_name::<#param>(),
                    std::mem::align_of::<#param>(),
                    std::any::type_name::<#new_type>(),
                    std::mem::align_of::<#new_type>()
                );
            }
        }
    });

    let replacement_logic = match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_replacements = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    if let Some(index) = type_params.iter().position(|&p| type_is_param(field_type, &[p])) {
                        if index < num_params {
                            let f_ident = format_ident!("f{}", index);
                            quote! {
                                let old = std::ptr::read(&self.#field_name as *const #field_type);
                                let new = #f_ident(old);
                                std::ptr::write(&mut self.#field_name as *mut #field_type as *mut _, new);
                            }
                        } else {
                            quote! {}
                        }
                    } else if type_contains_param(field_type, type_params) {
                        let method_name = format_ident!("replace_in_place_{}", num_params);
                        let f_idents: Vec<_> = (0..num_params).map(|i| format_ident!("f{}", i)).collect();
                        quote! {
                            self.#field_name = <#field_type as replace_type_in_place::ReplaceInPlace>::#method_name(self.#field_name, #(#f_idents),*);
                        }
                    } else {
                        quote! {}
                    }
                });
                quote! {
                    let mut self_wrapped = std::mem::ManuallyDrop::new(self);
                    #(#field_replacements)*
                    unsafe {
                        std::mem::transmute::<std::mem::ManuallyDrop<#name<#(#type_params),*>>, #name<#(#type_params),*>>(self_wrapped)
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let field_replacements = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = syn::Index::from(i);
                    let field_type = &f.ty;
                    if let Some(param_index) = type_params.iter().position(|&p| type_is_param(field_type, &[p])) {
                        if param_index < num_params {
                            let f_ident = format_ident!("f{}", param_index);
                            quote! {
                                let old = std::ptr::read(&self_wrapped.#index as *const #field_type);
                                let new = #f_ident(old);
                                std::ptr::write(&mut self_wrapped.#index as *mut #field_type as *mut _, new);
                            }
                        } else {
                            quote! {}
                        }
                    } else if type_contains_param(field_type, type_params) {
                        let method_name = format_ident!("replace_in_place_{}", num_params);
                        let f_idents: Vec<_> = (0..num_params).map(|i| format_ident!("f{}", i)).collect();
                        quote! {
                            self_wrapped.#index = <#field_type as replace_type_in_place::ReplaceInPlace>::#method_name(self_wrapped.#index, #(#f_idents),*);
                        }
                    } else {
                        quote! {}
                    }
                });
                quote! {
                    let mut self_wrapped = std::mem::ManuallyDrop::new(self);
                    #(#field_replacements)*
                    unsafe {
                        std::mem::transmute::<std::mem::ManuallyDrop<#name<#(#type_params),*>>, #name<#(#type_params),*>>(self_wrapped)
                    }
                }
            }
            Fields::Unit => quote! { #name },
        },
        Data::Enum(ref data) => {
            let variant_replacements = data.variants.iter().map(|v| {
                let variant_name = &v.ident;
                match &v.fields {
                    Fields::Named(ref fields) => {
                        let field_replacements = fields.named.iter().map(|f| {
                            let field_name = &f.ident;
                            let field_type = &f.ty;
                            if let Some(index) = type_params.iter().position(|&p| type_is_param(field_type, &[p])) {
                                if index < num_params {
                                    let f_ident = format_ident!("f{}", index);
                                    quote! {
                                        let old = std::ptr::read(#field_name as *const #field_type);
                                        let new = #f_ident(old);
                                        std::ptr::write(#field_name as *mut #field_type as *mut _, new);
                                    }
                                } else {
                                    quote! {}
                                }
                            } else if type_contains_param(field_type, type_params) {
                                let method_name = format_ident!("replace_in_place_{}", num_params);
                                let f_idents: Vec<_> = (0..num_params).map(|i| format_ident!("f{}", i)).collect();
                                quote! {
                                    *#field_name = <#field_type as replace_type_in_place::ReplaceInPlace>::#method_name(std::mem::replace(#field_name, std::mem::uninitialized()), #(#f_idents),*);
                                }
                            } else {
                                quote! {}
                            }
                        });
                        let field_names = fields.named.iter().map(|f| &f.ident);
                        quote! {
                            #name::#variant_name { #(ref mut #field_names),* } => {
                                #(#field_replacements)*
                            }
                        }
                    },
                    Fields::Unnamed(ref fields) => {
                        let field_replacements = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let field_name = format_ident!("field{}", i);
                            let field_type = &f.ty;
                            if let Some(index) = type_params.iter().position(|&p| type_is_param(field_type, &[p])) {
                                if index < num_params {
                                    let f_ident = format_ident!("f{}", index);
                                    quote! {
                                        let old = std::ptr::read(#field_name as *const #field_type);
                                        let new = #f_ident(old);
                                        std::ptr::write(#field_name as *mut #field_type as *mut _, new);
                                    }
                                } else {
                                    quote! {}
                                }
                            } else if type_contains_param(field_type, type_params) {
                                let method_name = format_ident!("replace_in_place_{}", num_params);
                                let f_idents: Vec<_> = (0..num_params).map(|i| format_ident!("f{}", i)).collect();
                                quote! {
                                    *#field_name = <#field_type as replace_type_in_place::ReplaceInPlace>::#method_name(std::mem::replace(#field_name, std::mem::uninitialized()), #(#f_idents),*);
                                }
                            } else {
                                quote! {}
                            }
                        });
                        let field_names = (0..fields.unnamed.len()).map(|i| format_ident!("field{}", i));
                        quote! {
                            #name::#variant_name(#(ref mut #field_names),*) => {
                                #(#field_replacements)*
                            }
                        }
                    },
                    Fields::Unit => quote! {
                        #name::#variant_name => {}
                    },
                }
            });
            quote! {
                let mut self_wrapped = std::mem::ManuallyDrop::new(self);
                match &mut *self_wrapped {
                    #(#variant_replacements)*
                }
                unsafe {
                    std::mem::transmute::<std::mem::ManuallyDrop<#name<#(#type_params),*>>, #name<#(#type_params),*>>(self_wrapped)
                }
            }
        }
        Data::Union(_) => unimplemented!("Unions are not supported"),
    };

    quote! {
        #(#checks)*
        // This is safe because we are checking size and alignment of the types above.
        unsafe {
            #replacement_logic
        }
    }
}
