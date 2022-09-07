/*!
A simple crate for generation static noise at compile time.

# Example

```rust
let random_company_name: &'static str = static_noise::company_name!();
let random_super_power: &'static str = static_noise::super_power!();
let random_uuid: &'static str = static_noise::uuid!();
```
*/

use proc_macro::TokenStream;
use rand::{thread_rng, Rng};
use uuid::Uuid;

static COMPANY_NAMES: &str = include_str!("../random-data/company-names.json");

/// returns a random name for a company
#[proc_macro]
pub fn company_name(_: TokenStream) -> TokenStream {
    let company_names: Vec<String> = serde_json::from_str(COMPANY_NAMES).unwrap();
    let mut rng = thread_rng();
    format!("\"{}\"", company_names[(rng.gen::<u16>() % company_names.len() as u16 - 1) as usize]).parse().unwrap()
}

static SUPERPOWERS: &str = include_str!("../random-data/company-names.json");

/// returns a random superpower
#[proc_macro]
pub fn superpower(_: TokenStream) -> TokenStream {
    let super_powers: Vec<String> = serde_json::from_str(SUPERPOWERS).unwrap();
    let mut rng = thread_rng();
    format!("\"{}\"", super_powers[(rng.gen::<u16>() % super_powers.len() as u16 - 1) as usize]).parse().unwrap()
}

/// returns a random uuid
#[proc_macro]
pub fn uuid(_: TokenStream) -> TokenStream {
    let uuid = Uuid::new_v4();
    format!("\"{}\"", uuid).parse().unwrap()
}

