//! **음성 대조군** — 손 표본이 고장이면 여기서 드러난다.
//! tree-sitter 와 다른 파서(`syn`)로 같은 규칙을 적용해 선언을 센다.
use syn::{Item, ImplItem, TraitItem, Type};

fn 타입이름(t: &Type) -> Option<String> {
    match t {
        Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        Type::Reference(r) => 타입이름(&r.elem),
        _ => None,
    }
}

fn 훑는다(items: &[Item], chain: &str, out: &mut Vec<(String, String, String)>) {
    for it in items {
        match it {
            Item::Fn(f) => out.push((chain.into(), f.sig.ident.to_string(), "function".into())),
            Item::Struct(s) => out.push((chain.into(), s.ident.to_string(), "struct".into())),
            Item::Enum(e) => out.push((chain.into(), e.ident.to_string(), "enum".into())),
            Item::Union(u) => out.push((chain.into(), u.ident.to_string(), "union".into())),
            Item::Type(t) => out.push((chain.into(), t.ident.to_string(), "type_alias".into())),
            Item::Const(c) => out.push((chain.into(), c.ident.to_string(), "const".into())),
            Item::Static(s) => out.push((chain.into(), s.ident.to_string(), "static".into())),
            Item::Macro(m) => { if let Some(id) = &m.ident {
                out.push((chain.into(), id.to_string(), "macro".into())); } }
            Item::Trait(t) => {
                out.push((chain.into(), t.ident.to_string(), "trait".into()));
                let c = if chain == "-" { t.ident.to_string() } else { format!("{chain}.{}", t.ident) };
                for ti in &t.items {
                    // **본문 있는 것만** — 규칙 ⑤
                    if let TraitItem::Fn(f) = ti { if f.default.is_some() {
                        out.push((c.clone(), f.sig.ident.to_string(), "function".into())); } }
                    if let TraitItem::Type(ty) = ti {
                        out.push((c.clone(), ty.ident.to_string(), "type_alias".into())); }
                    if let TraitItem::Const(cc) = ti {
                        out.push((c.clone(), cc.ident.to_string(), "const".into())); }
                }
            }
            Item::Mod(m) => {
                out.push((chain.into(), m.ident.to_string(), "module".into()));
                if let Some((_, inner)) = &m.content {
                    let c = if chain == "-" { m.ident.to_string() } else { format!("{chain}.{}", m.ident) };
                    훑는다(inner, &c, out);
                }
            }
            Item::Impl(i) => {
                let Some(t) = 타입이름(&i.self_ty) else { continue };
                let c = if chain == "-" { t.clone() } else { format!("{chain}.{t}") };
                for ii in &i.items {
                    match ii {
                        ImplItem::Fn(f) => out.push((c.clone(), f.sig.ident.to_string(), "function".into())),
                        ImplItem::Const(cc) => out.push((c.clone(), cc.ident.to_string(), "const".into())),
                        ImplItem::Type(ty) => out.push((c.clone(), ty.ident.to_string(), "type_alias".into())),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    for path in std::env::args().skip(1) {
        let Ok(src) = std::fs::read_to_string(&path) else { continue };
        let Ok(f) = syn::parse_file(&src) else { eprintln!("파싱 실패 {path}"); continue };
        let mut out = Vec::new();
        훑는다(&f.items, "-", &mut out);
        for (c, n, k) in out { println!("{path}\t{c}\t{n}\t{k}"); }
    }
}
