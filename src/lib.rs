pub mod filter;
pub mod pix_fmt;

pub mod items;

fn collect_string(mut total: String, next: &str) -> String {
    if total.is_empty() {
        total = next.to_string();
    } else {
        total = total + " " + next;
    }
    total
}
