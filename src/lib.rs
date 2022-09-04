use proc_macro::TokenStream;
use rand::{thread_rng, Rng};
use uuid::Uuid;

static COMPANY_NAMES: &str = include_str!("../random-data/company-names.json");

#[proc_macro]
pub fn company_name(_: TokenStream) -> TokenStream {
    let company_names: Vec<String> = serde_json::from_str(COMPANY_NAMES).unwrap();
    let mut rng = thread_rng();
    format!("\"{}\"", company_names[(rng.gen::<u16>() % company_names.len() as u16 - 1) as usize]).parse().unwrap()
}

static SUPERPOWERS: &str = include_str!("../random-data/company-names.json");

#[proc_macro]
pub fn superpower(_: TokenStream) -> TokenStream {
    let super_powers: Vec<String> = serde_json::from_str(SUPERPOWERS).unwrap();
    let mut rng = thread_rng();
    format!("\"{}\"", super_powers[(rng.gen::<u16>() % super_powers.len() as u16 - 1) as usize]).parse().unwrap()
}

#[proc_macro]
pub fn uuid(_: TokenStream) -> TokenStream {
    let uuid = Uuid::new_v4();
    format!("\"{}\"", uuid).parse().unwrap()
}

