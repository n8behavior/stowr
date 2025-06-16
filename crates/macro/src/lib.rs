use heck::ToUpperCamelCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, FnArg, ImplItem, ItemImpl, ItemStruct, Pat, PatType, Type};

/// Attribute macro to generate domain boilerplate:
/// - Zero-variant tag enum + `RepositoryId` alias
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

/// Marker attribute: flags a method for command/event generation
#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Attribute macro to generate command/event enums and Aggregate implementation
/// from an `impl` block with `#[command]` methods.
#[proc_macro_attribute]
pub fn domain_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    // Extract the implementor type, e.g. `Foo`
    let self_ty = match &*input.self_ty {
        Type::Path(tp) => tp.path.segments.last().unwrap().ident.clone(),
        _ => panic!("#[domain_impl] only supports inherent impls on a type"),
    };

    // Prepare lists for generated code
    let mut cmd_variants = Vec::new();
    let mut evt_variants = Vec::new();
    let mut handle_arms = Vec::new();
    let mut apply_arms = Vec::new();

    // Iterate methods to find #[command]
    for item in &input.items {
        if let ImplItem::Fn(m) = item {
            let is_cmd = m.attrs.iter().any(|a| a.path().is_ident("command"));
            if !is_cmd {
                continue;
            }

            let method = &m.sig.ident;
            let variant_name = format_ident!("{}", method.to_string().to_upper_camel_case());

            // Collect argument names and types
            let mut fields = Vec::new();
            for arg in &m.sig.inputs {
                if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                    if let Pat::Ident(pi) = pat.as_ref() {
                        fields.push((pi.ident.clone(), ty));
                    }
                }
            }
            let names: Vec<_> = fields.iter().map(|(ident, _)| ident).collect();
            let types: Vec<_> = fields.iter().map(|(_, ty)| ty).collect();

            // Generate command and event enum variants
            cmd_variants.push(quote! { #variant_name { #(#names: #types),* } });
            evt_variants.push(quote! { #variant_name { #(#names: #types),* } });

            // Generate match arms for handle_command
            let cmd_enum = format_ident!("{}Command", self_ty);
            let evt_enum = format_ident!("{}Event", self_ty);
            handle_arms.push(quote! {
                #cmd_enum::#variant_name { #(#names),* } => {
                    let mut agg = self.clone();
                    agg.#method(#(#names.clone()),*);
                    vec![#evt_enum::#variant_name { #(#names),* }]
                }
            });

            // Generate match arms for apply_event
            apply_arms.push(quote! {
                #evt_enum::#variant_name { #(#names),* } => {
                    self.#method(#(#names.clone()),*);
                }
            });
        }
    }

    // Final enum names
    let cmd_enum = format_ident!("{}Command", self_ty);
    let evt_enum = format_ident!("{}Event", self_ty);

    // Assemble the expanded code
    let expanded = quote! {
        #input

        #[derive(Clone, Debug)]
        pub enum #cmd_enum {
            #(#cmd_variants),*
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub enum #evt_enum {
            #(#evt_variants),*
        }

        impl Aggregate for #self_ty {
            type Command = #cmd_enum;
            type Event   = #evt_enum;
            type Error   = crate::AggregateError;

            fn handle_command(&self, cmd: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
                Ok(match cmd {
                    #(#handle_arms),*
                })
            }

            fn apply_event(&mut self, evt: &Self::Event) {
                match evt {
                    #(#apply_arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
