use serde::Serialize;
use ts_rs::TS;

#[allow(non_camel_case_types, dead_code)]
#[derive(TS, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Struct {
    kebab_cased: usize,
}

#[test]
fn kebab_case() {
    let out = <Struct as TS>::decl();
    assert_eq!(out, "interface Struct { \"kebab-cased\": number, }");
}
