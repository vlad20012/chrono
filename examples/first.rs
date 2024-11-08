use chrono::format::formatting::get_wiki_page_for_data;
use chrono::Utc;
use chrono::DateTime;

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let wiki_page = get_wiki_page_for_data(now);
    println!("{}", wiki_page);
}