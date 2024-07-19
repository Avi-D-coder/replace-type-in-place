use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, TypeParam
};

#[proc_macro_derive(Replace)]
pub fn derive_replace(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let type_params = generics.type_params().collect::<Vec<_>>();
    let replace_impls = generate_replace_impls(name, &type_params, &input.data, generics);
    let expanded = quote! {
        #(#replace_impls)*
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(ReplaceInPlace)]
pub fn derive_replace_in_place(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let type_params = generics.type_params().collect::<Vec<_>>();
    let replace_in_place_impls = generate_replace_in_place_impls(name, &type_params, &input.data, generics);
    let expanded = quote! {
        #(#replace_in_place_impls)*
    };
    TokenStream::from(expanded)
}

fn generate_replace_impls(
    name: &syn::Ident,
    type_params: &[&TypeParam],
    data: &Data,
    generics: &syn::Generics,
) -> Vec<proc_macro2::TokenStream> {
    type_params
        .iter()
        .map(|&param| {
            let param_name = &param.ident;
            let replace_fields = generate_replace_fields(name, data, param_name);
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            
            quote! {
                impl #impl_generics Replace<#param_name> for #name #ty_generics #where_clause {
                    type OutputSelf<New> = #name<New>;
                    fn replace<New>(self, f: &mut impl FnMut(#param_name) -> New) -> Self::OutputSelf<New> {
                        #replace_fields
                    }
                }
            }
        })
        .collect()
}

fn generate_replace_in_place_impls(
    name: &syn::Ident,
    type_params: &[&TypeParam],
    data: &Data,
    generics: &syn::Generics,
) -> Vec<proc_macro2::TokenStream> {
    type_params
        .iter()
        .map(|&param| {
            let param_name = &param.ident;
            let replace_in_place_fields = generate_replace_in_place_fields(name, data, param_name);
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            
            quote! {
                impl #impl_generics ReplaceInPlace<#param_name> for #name #ty_generics #where_clause {
                    type OutputSelf<New> = #name<New>;
                    fn replace_in_place<New>(self, f: &mut impl FnMut(#param_name) -> New) -> <Self as ReplaceInPlace<#param_name>>::OutputSelf<New> {
                        #replace_in_place_fields
                    }
                }
            }
        })
        .collect()
}

fn generate_replace_fields(
    name: &syn::Ident,
    data: &Data,
    param_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let alignment_check = quote! {
        if std::mem::align_of::<#param_name>() != std::mem::align_of::<New>() {
            panic!(
                "The Old type has a different alignment than the New type you tried to replace it with: \n\
                Old: {} alignment: {}\n\
                New: {} alignment: {}",
                std::any::type_name::<#param_name>(),
                std::mem::align_of::<#param_name>(),
                std::any::type_name::<New>(),
                std::mem::align_of::<New>()
            );
        }
    };

    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_replacements = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    quote! { #field_name: <_ as Replace<#param_name>>::replace(self.#field_name, f), }
                });
                quote! {
                    #alignment_check
                    #name { #(#field_replacements)* }
                }
            }
            Fields::Unnamed(ref fields) => {
                let field_replacements = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! { <_ as Replace<#param_name>>::replace(self.#index, f), }
                });
                quote! {
                    #alignment_check
                    #name(#(#field_replacements)*)
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
                            quote! { #field_name: <_ as Replace<#param_name>>::replace(#field_name, f), }
                        });

                        let field_replacements_ = field_replacements.clone();
                        quote! {
                            #name::#variant_name { #(#field_replacements)* } => #name::#variant_name { #(#field_replacements_)* },
                        }
                    },
                    Fields::Unnamed(ref fields) => {
                        let field_names: Vec<syn::Ident> = (0..fields.unnamed.len())
                            .map(|i| format_ident!("field{}", i))
                            .collect();
                        let field_replacements = field_names.iter().map(|field_name| {
                            quote! { <_ as Replace<#param_name>>::replace(#field_name, f), }
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
                #alignment_check
                match self {
                    #(#variant_replacements)*
                }
            }
        }
        Data::Union(_) => unimplemented!("Unions are not supported"),
    }
}

fn generate_replace_in_place_fields(
    name: &syn::Ident,
    data: &Data,
    param_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let checks = quote! {
        if std::mem::size_of::<#param_name>() < std::mem::size_of::<New>() {
            panic!(
                "The Old type is smaller than the New type you tried to replace it with: \n\
                Old: {} size: {}\n\
                New: {} size: {}",
                std::any::type_name::<#param_name>(),
                std::mem::size_of::<#param_name>(),
                std::any::type_name::<New>(),
                std::mem::size_of::<New>()
            );
        }

        if std::mem::align_of::<#param_name>() != std::mem::align_of::<New>() {
            panic!(
                "The Old type has a different alignment than the New type you tried to replace it with: \n\
                Old: {} alignment: {}\n\
                New: {} alignment: {}",
                std::any::type_name::<#param_name>(),
                std::mem::align_of::<#param_name>(),
                std::any::type_name::<New>(),
                std::mem::align_of::<New>()
            );
        }

        let size_of_old_self = std::mem::size_of::<Self>();
        let size_of_new_self = std::mem::size_of::<<Self as ReplaceInPlace<#param_name>>::OutputSelf<New>>();

        if size_of_old_self != size_of_new_self {
            panic!(
                "The size of the {}<Old> is not the same as the size of the {}<New>: \n\
                {}<Old> size: {}\n\
                {}<New> size: {}",
                stringify!(#name), stringify!(#name),
                stringify!(#name), size_of_old_self,
                stringify!(#name), size_of_new_self
            );
        }

        let align_of_old_self = std::mem::align_of::<Self>();
        let align_of_new_self = std::mem::align_of::<<Self as ReplaceInPlace<#param_name>>::OutputSelf<New>>();

        if align_of_old_self != align_of_new_self {
            panic!(
                "The alignment of the {}<Old> is not the same as the alignment of the {}<New>: \n\
                {}<Old> alignment: {}\n\
                {}<New> alignment: {}",
                stringify!(#name), stringify!(#name),
                stringify!(#name), align_of_old_self,
                stringify!(#name), align_of_new_self
            );
        }
    };

    let replacement_logic = match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_replacements = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    quote! { #field_name: <_ as ReplaceInPlace<#param_name>>::replace_in_place(self.#field_name, f), }
                });
                quote! { #name { #(#field_replacements)* } }
            }
            Fields::Unnamed(ref fields) => {
                let field_replacements = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! { <_ as ReplaceInPlace<#param_name>>::replace_in_place(self.#index, f), }
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
                            quote! { #field_name: <_ as ReplaceInPlace<#param_name>>::replace_in_place(#field_name, f), }
                        });
                        let field_replacements_ = field_replacements.clone();
                        quote! {
                            #name::#variant_name { #(#field_replacements)* } => #name::#variant_name { #(#field_replacements_)* },
                        }
                    },
                    Fields::Unnamed(ref fields) => {
                        let field_names: Vec<syn::Ident> = (0..fields.unnamed.len())
                            .map(|i| format_ident!("field{}", i))
                            .collect();
                        let field_replacements = field_names.iter().map(|field_name| {
                            quote! { <_ as ReplaceInPlace<#param_name>>::replace_in_place(#field_name, f), }
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
    };

    quote! {
        #checks
        #replacement_logic
    }
}
