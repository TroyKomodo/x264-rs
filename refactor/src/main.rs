use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use proc_macro2::Span;
use syn::{Token, parse_quote};

fn find_all_rust_files(target_folder: &Path) -> Vec<File> {
    let mut rust_files = Vec::new();
    let files = target_folder.read_dir().unwrap();
    for file in files {
        let file_path = file.unwrap().path();
        if file_path.extension().is_some_and(|ext| ext == "rs") {
            rust_files.push(File {
                syn_file: syn::parse_str(&std::fs::read_to_string(&file_path).unwrap()).unwrap(),
                path: file_path,
            });
        } else if file_path.is_dir() {
            rust_files.extend(find_all_rust_files(&file_path));
        }
    }

    rust_files
}

struct File {
    path: PathBuf,
    syn_file: syn::File,
}

fn main() {
    let target_folder = std::env::args().nth(1).unwrap();
    let target_folder = Path::new(&target_folder);

    let out_folder = std::env::args().nth(2).unwrap();
    let out_folder = Path::new(&out_folder);

    println!("Target folder: {}", target_folder.display());

    let mut rust_files = find_all_rust_files(target_folder);

    #[derive(Hash, Eq, PartialEq)]
    enum DefType {
        Struct,
        Enum,
        Union,
        Static,
        Type,
        Const,
    }

    #[derive(Hash, Eq, PartialEq)]
    struct DefKey {
        ident: String,
        def_type: DefType,
    }

    struct Def {
        file: PathBuf,
        item: syn::Item,
    }

    let mut seen_defs = HashMap::new();

    let mut module_c2_defs = HashMap::new();

    for file in &mut rust_files {
        let mut has_any = false;
        let mut c2_defs = Vec::new();
        file.syn_file.items.retain_mut(|root_item| match root_item {
            syn::Item::Struct(item) => {
                if item.ident.to_string().starts_with("C2Rust") {
                    c2_defs.push(root_item.clone());
                    return false;
                }

                item.vis = syn::Visibility::Public(syn::parse_str("pub").unwrap());

                seen_defs.insert(
                    DefKey {
                        ident: item.ident.to_string(),
                        def_type: DefType::Struct,
                    },
                    Def {
                        file: file.path.clone(),
                        item: root_item.clone(),
                    },
                );
                has_any = true;
                false
            }
            syn::Item::Enum(item) => {
                if item.ident.to_string().starts_with("C2Rust") {
                    c2_defs.push(root_item.clone());
                    return false;
                }

                item.vis = syn::Visibility::Public(syn::parse_str("pub").unwrap());

                seen_defs.insert(
                    DefKey {
                        ident: item.ident.to_string(),
                        def_type: DefType::Enum,
                    },
                    Def {
                        file: file.path.clone(),
                        item: root_item.clone(),
                    },
                );
                has_any = true;
                false
            }
            syn::Item::Union(item) => {
                if item.ident.to_string().starts_with("C2Rust") {
                    c2_defs.push(root_item.clone());
                    return false;
                }

                item.vis = syn::Visibility::Public(syn::parse_str("pub").unwrap());

                seen_defs.insert(
                    DefKey {
                        ident: item.ident.to_string(),
                        def_type: DefType::Union,
                    },
                    Def {
                        file: file.path.clone(),
                        item: root_item.clone(),
                    },
                );
                has_any = true;
                false
            }
            syn::Item::Static(item) => {
                if item.ident.to_string().starts_with("C2Rust") {
                    c2_defs.push(root_item.clone());
                    return false;
                }

                item.vis = syn::Visibility::Public(syn::parse_str("pub").unwrap());

                seen_defs.insert(
                    DefKey {
                        ident: item.ident.to_string(),
                        def_type: DefType::Static,
                    },
                    Def {
                        file: file.path.clone(),
                        item: root_item.clone(),
                    },
                );
                has_any = true;
                false
            }
            syn::Item::Type(item) => {
                if item.ident.to_string().starts_with("C2Rust") {
                    c2_defs.push(root_item.clone());
                    return false;
                }

                item.vis = syn::Visibility::Public(syn::parse_str("pub").unwrap());

                seen_defs.insert(
                    DefKey {
                        ident: item.ident.to_string(),
                        def_type: DefType::Type,
                    },
                    Def {
                        file: file.path.clone(),
                        item: root_item.clone(),
                    },
                );
                has_any = true;
                false
            }
            syn::Item::Const(item) => {
                if item.ident.to_string().starts_with("C2Rust") {
                    c2_defs.push(root_item.clone());
                    return false;
                }

                item.vis = syn::Visibility::Public(syn::parse_str("pub").unwrap());

                seen_defs.insert(
                    DefKey {
                        ident: item.ident.to_string(),
                        def_type: DefType::Const,
                    },
                    Def {
                        file: file.path.clone(),
                        item: root_item.clone(),
                    },
                );
                has_any = true;
                false
            }
            _ => true,
        });

        if has_any {
            module_c2_defs.insert(file.path.clone(), c2_defs);
        }
    }

    for file in &mut rust_files {
        file.syn_file.items.iter_mut().for_each(|item| {
            if let syn::Item::ForeignMod(item) = item {
                item.items.retain(|item| match item {
                    syn::ForeignItem::Type(_) => false,
                    syn::ForeignItem::Static(item) => !seen_defs.contains_key(&DefKey {
                        ident: item.ident.to_string(),
                        def_type: DefType::Static,
                    }),
                    _ => true,
                });
            }
        });
    }

    let mut seen_defs = seen_defs
        .into_values()
        .map(|def| (def.file, def.item))
        .into_group_map();

    struct Module {
        sub_modules: HashMap<syn::Ident, Module>,
        items: Vec<syn::Item>,
        c2_defs: Vec<syn::Item>,
    }

    let mut root_module = Module {
        sub_modules: HashMap::new(),
        items: Vec::new(),
        c2_defs: Vec::new(),
    };

    for (file, c2_defs) in module_c2_defs {
        let defs = seen_defs.remove(&file).unwrap_or_default();

        let relative_path = file.strip_prefix(target_folder).unwrap();
        let mut sub_module = &mut root_module;
        let accessors = relative_path.ancestors().collect::<Vec<_>>();
        for path in accessors.into_iter().rev() {
            let Some(file_name) = path.file_name() else {
                continue;
            };

            let module_name = file_name.to_str().unwrap().trim_end_matches(".rs");
            sub_module = sub_module
                .sub_modules
                .entry(syn::Ident::new(
                    &format!("{}_file", module_name),
                    Span::call_site(),
                ))
                .or_insert_with(|| Module {
                    sub_modules: HashMap::new(),
                    items: Vec::new(),
                    c2_defs: Vec::new(),
                });
        }

        sub_module.items.extend(defs);
        sub_module.c2_defs.extend(c2_defs);
    }

    fn make_module(module: &Module) -> proc_macro2::TokenStream {
        let items = &module.items;
        let c2_defs = &module.c2_defs;
        let sub_modules = module.sub_modules.iter().map(|(ident, sub_module)| {
            let tokens = make_module(sub_module);
            quote::quote! {
                pub use #ident::*;
                pub mod #ident {
                    use super::*;
                    #tokens
                }
            }
        });

        quote::quote! {
            #(#items)*
            #(#sub_modules)*
            use c2_defs::*;
            mod c2_defs {
                use super::*;
                #(#c2_defs)*
            }
        }
    }

    for file in &mut rust_files {
        file.syn_file.items.insert(0, parse_quote! {
            use crate::types::*;
        });

        let pretty_file = prettyplease::unparse(&file.syn_file);
        let relative_path = file.path.strip_prefix(target_folder).unwrap();
        let new_path = out_folder.join(relative_path);
        if new_path.parent().is_some_and(|parent| !parent.exists()) {
            std::fs::create_dir_all(new_path.parent().unwrap()).unwrap();
        }

        std::fs::write(&new_path, pretty_file).unwrap();
    }

    let tokens = syn::parse2::<syn::File>(make_module(&root_module)).unwrap();

    std::fs::write(out_folder.join("types.rs"), prettyplease::unparse(&tokens)).unwrap();

    println!("Done!");
}
