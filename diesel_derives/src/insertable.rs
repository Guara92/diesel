use syn;
use quote;
use proc_macro2::Span;

use field::*;
use model::*;
use util::*;

pub fn derive(item: syn::DeriveInput) -> Result<quote::Tokens, Diagnostic> {
    let model = Model::from_item(&item)?;

    if model.fields().is_empty() {
        return Err(Span::call_site()
            .error("Cannot derive Insertable for unit structs")
            .help(format!(
                "Use `insert_into({}::table).default_values()` if you want `DEFAULT VALUES`",
                model.table_name()
            )));
    }

    let table_name = model.table_name();
    let struct_name = item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let mut ref_generics = item.generics.clone();
    ref_generics.params.push(parse_quote!('insert));
    let (ref_generics, ..) = ref_generics.split_for_impl();

    let (direct_field_ty, direct_field_assign): (Vec<_>, Vec<_>) = model
        .fields()
        .iter()
        .map(|f| {
            (
                (field_ty(f, table_name, quote!())),
                (field_expr(f, table_name, None)),
            )
        })
        .unzip();

    let (ref_field_ty, ref_field_assign): (Vec<_>, Vec<_>) = model
        .fields()
        .iter()
        .map(|f| {
            (
                (field_ty(f, table_name, quote!(&'insert))),
                (field_expr(f, table_name, Some(quote!(&)))),
            )
        })
        .unzip();

    Ok(wrap_in_dummy_mod(
        model.dummy_mod_name("insertable"),
        quote! {
            use self::diesel::insertable::Insertable;
            use self::diesel::query_builder::UndecoratedInsertRecord;
            use self::diesel::prelude::*;

            impl #impl_generics Insertable<#table_name::table> for #struct_name #ty_generics
                #where_clause
            {
                type Values = <(#(#direct_field_ty,)*) as Insertable<#table_name::table>>::Values;

                fn values(self) -> Self::Values {
                    (#(#direct_field_assign,)*).values()
                }
            }

            impl #ref_generics Insertable<#table_name::table>
                for &'insert #struct_name #ty_generics
            #where_clause
            {
                type Values = <(#(#ref_field_ty,)*) as Insertable<#table_name::table>>::Values;

                fn values(self) -> Self::Values {
                    (#(#ref_field_assign,)*).values()
                }
            }

            impl #impl_generics UndecoratedInsertRecord<#table_name::table>
                for #struct_name #ty_generics
            #where_clause
            {
            }
        },
    ))
}

fn field_ty(field: &Field, table_name: syn::Ident, life_time: quote::Tokens) -> syn::Type {
    if field.has_flag("embed") {
        let field_ty = &field.ty;
        parse_quote!(#life_time #field_ty)
    } else {
        let inner_ty = inner_of_option_ty(&field.ty);
        let column_name = field.column_name();
        parse_quote!(
            std::option::Option<diesel::dsl::Eq<
                #table_name::#column_name,
                #life_time #inner_ty,
            >>
        )
    }
}

fn field_expr(
    field: &Field,
    table_name: syn::Ident,
    life_time: Option<quote::Tokens>,
) -> syn::Expr {
    let field_access = field.name.access();
    if field.has_flag("embed") {
        parse_quote!(#life_time self#field_access)
    } else {
        let column_name = field.column_name();
        let column: syn::Expr = parse_quote!(#table_name::#column_name);
        if is_option_ty(&field.ty) {
            if life_time.is_some() {
                parse_quote!(self#field_access.as_ref().map(|x| #column.eq(x)))
            } else {
                parse_quote!(self#field_access.map(|x| #column.eq(x)))
            }
        } else {
            parse_quote!(std::option::Option::Some(#column.eq(#life_time self#field_access)))
        }
    }
}
