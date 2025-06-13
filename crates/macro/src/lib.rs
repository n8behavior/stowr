use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, ItemStruct};

/// Attribute macro to generate domain boilerplate:
/// - Zero‑variant tag enum + `RepositoryId` alias
/// - Struct with `id` field prepended
/// - `new(...)` constructor using `Into` for each field
#[proc_macro_attribute]
pub fn domain(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let vis = &input.vis;

    let tag = format_ident!("{}Tag", name);
    let id = format_ident!("{}Id", name);
    let repo_trait = format_ident!("{}Repository", name);
    let repo_alias = format_ident!("{}Repo", name);

    let fields = match &input.fields {
        Fields::Named(named) => &named.named,
        _ => panic!("#[domain] only supports structs with named fields"),
    };
    let names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let tys: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let expanded = quote! {
        // ANCHOR: #name_domain
        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #vis enum #tag {}
        #vis type #id = RepositoryId<#tag>;

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #vis struct #name {
            pub id: #id,
            #(pub #names: #tys),*
        }

        impl #name {
            /// Create a new instance with converted fields
            pub fn new(
                id: #id,
                #(#names: impl Into<#tys>),*
            ) -> Self {
                #name {
                    id,
                    #(#names: #names.into()),*
                }
            }
        }

        // Repository helper trait for this domain
        pub trait #repo_trait: Repository<Entity = #name, Id = #id> + Send + Sync {}
        impl<T> #repo_trait for T where T: Repository<Entity = #name, Id = #id> + Send + Sync {}
        /// Arc-ed trait object alias for repositories of this domain
        pub type #repo_alias = std::sync::Arc<dyn #repo_trait>;
        // ANCHOR_END: #name_domain
    };

    TokenStream::from(expanded)
}
