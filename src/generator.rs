use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::fs::{create_dir_all, read, read_dir, remove_dir_all, write, File};
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

fn main() {
    struct Feature {
        name: String,
        children: Vec<String>,
    }

    let mut variants = Vec::new();
    let mut features = Vec::new();
    let mut imports = Vec::new();

    remove_dir_all("src/generated").unwrap();
    create_dir_all("src/generated").unwrap();

    let cargo_file = String::from_utf8(read("Cargo.toml").unwrap()).unwrap();
    let start_i = cargo_file.find("\"base64\"]").unwrap() + 10;

    write("Cargo.toml", cargo_file[..start_i].to_owned() + "\n").unwrap();

    let mut cargo_toml = File::options().append(true).open("Cargo.toml").unwrap();

    let mut generate = |prefix: &str, dir: &str| {
        let mut function_mods = Vec::new();

        let feature_name = prefix.to_case(Case::Snake);
        let feature_ident = to_ident(&feature_name);
        let mut collection_feature = Feature {
            name: feature_name.clone(),
            children: Vec::new(),
        };

        let result = read_dir(dir);
        let mut paths: Vec<_> = result
            .expect(dir)
            .map(|result| result.unwrap())
            .map(|entry| entry.path())
            .map(|path_buf| path_buf.to_str().unwrap().to_owned())
            .collect();

        paths.sort();

        create_dir_all(format!("src/generated/{}", feature_name)).unwrap();

        for path in paths {
            let path = path.replace('\\', "/");
            let file_name = path.split('/').last().unwrap();
            if !file_name.ends_with(".svg") {
                continue;
                //panic!("never happens?");
            }
            let icon_name = file_name.split('.').next().unwrap();
            let name = prefix.to_owned() + "-" + icon_name;

            let contents = read(&path).expect(&path);
            let svg = std::str::from_utf8(&contents).unwrap();

            let Ok(dom) = tl::parse(&svg, tl::ParserOptions::default()) else {
                println!("Failed to parse svg: {}", path);
                continue;
            };
            let parser = dom.parser();
            let Some(svg_tag) = dom
                .query_selector("svg")
                .unwrap()
                .next()
                .and_then(|x| x.get(&parser).and_then(|y| y.as_tag()))
            else {
                println!("Failed to get svg: {}", path);
                continue;
            };

            let mut svg_props = svg_tag
                .attributes()
                .iter()
                .fold("&[".to_owned(), |acc, (k, v)| {
                    if let Some(v) = v {
                        format!("{}(\"{}\",\"{}\"),", acc, k, v)
                    } else {
                        acc
                    }
                });
            svg_props.pop();

            let mut inner_html = svg_tag.inner_html(parser);
            if let Some(x) = inner_html.find("<path") {
                inner_html = inner_html[x..].to_owned();
            }

            let svg_props = TokenStream::from_str((svg_props + "]").as_str()).expect(&path);
            let svg_tokens = TokenStream::from_str(&inner_html).expect(&path);

            let function_name = name.to_case(Case::Snake);
            let function_ident = to_ident(&function_name);

            let variant_name = name.to_case(Case::UpperCamel);
            let variant = to_ident(&variant_name);
            variants.push(quote! {
                #[cfg(feature = #variant_name)]
                #variant
            });

            collection_feature.children.push(variant_name.clone());
            features.push(Feature {
                name: variant_name.clone(),
                children: Vec::new(),
            });

            // Don't need when export separate mods #[cfg(feature = #variant_name)]
            let tokens = quote! {
                use leptos::*;
                use crate::Path;

                fn icon_path() -> Fragment {
                    view! {
                        <>
                            #svg_tokens
                        </>
                    }
                }

                pub const #variant: Path = Path {
                    path: icon_path,
                    props: #svg_props
                };
            };

            let output = tokens.to_string(); // reformat(tokens.to_string(), true).unwrap();

            write(
                format!("src/generated/{}/{}.rs", feature_name, function_name),
                output,
            )
            .unwrap();

            cargo_toml
                .write(format!("{} = []\n", variant_name).as_bytes())
                .unwrap();

            function_mods.push(quote! {
                #[cfg(feature = #variant_name)]
                mod #function_ident;
                #[cfg(feature = #variant_name)]
                pub use #function_ident::*;
            });
        }

        cargo_toml
            .write(format!("{} = [\n", feature_name).as_bytes())
            .unwrap();

        let children: Vec<_> = collection_feature
            .children
            .iter()
            .map(|f| {
                cargo_toml
                    .write(format!("\t\"{}\",\n", f).as_bytes())
                    .unwrap();

                quote! {
                    feature = #f
                }
            })
            .collect();

        cargo_toml.write("]\n".as_bytes()).unwrap();

        imports.push(quote! {
            cfg_if::cfg_if! {
                if #[cfg(any(
                    #(#children),*
                ))] {
                    mod #feature_ident;
                    pub use #feature_ident::*;
                }
            }
        });

        features.push(collection_feature);

        let tokens = quote! {
            #(#function_mods)*
        };

        let output = reformat(tokens.to_string(), true).unwrap();
        write(format!("src/generated/{}.rs", feature_name), output).unwrap();
    };

    generate("HeroiconsOutline", "heroicons/optimized/24/outline");
    generate("HeroiconsSolid", "heroicons/optimized/24/solid");
    generate("HeroiconsMiniSolid", "heroicons/optimized/20/solid");
    generate("Lucide", "lucide/icons");
    generate("FontAwesomeBrands", "fontawesome/svgs/brands");
    generate("FontAwesomeRegular", "fontawesome/svgs/regular");
    generate("FontAwesomeSolid", "fontawesome/svgs/solid");

    let tokens = quote! {
        #(#imports)*
    };

    let output = reformat(tokens.to_string(), false).unwrap();

    write("src/generated.rs", output).unwrap();

    features.sort_unstable_by_key(|feature| feature.name.clone());

    for feature in features {
        println!(
            r##"{} = [{}]"##,
            feature.name,
            feature
                .children
                .into_iter()
                .map(|c| format!(r##""{}""##, c))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn to_ident(string: &str) -> Ident {
    Ident::new(string, Span::call_site())
}

// https://github.com/rust-analyzer/rust-analyzer/blob/ada9e16537c22b490d13cdd54b9e1e4885856a4c/xtask/src/codegen.rs#L66-L78
fn reformat(text: impl std::fmt::Display, included: bool) -> Result<String, String> {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    write!(rustfmt.stdin.take().unwrap(), "{}", text).map_err(|e| e.to_string())?;
    let output = rustfmt.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let preamble = "Generated file, do not edit by hand, see `src/generator.rs`";
    let prefix = if included { "//" } else { "//!" };
    Ok(format!("{} {}\n\n{}", prefix, preamble, stdout))
}
