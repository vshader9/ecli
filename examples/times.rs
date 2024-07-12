use chrono::Utc;
fn main() {
    let now = Utc::now(); // Utc世界协调时时间，比中国北京慢8个小时
    println!("{}", now.timestamp());
}
