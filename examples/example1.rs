use reijou::セバスチャン;

セバスチャン! {
    わたくし std::env::args 様を使わせていただきますわ.
}

fn main() {
    for arg in args() {
        println!("{}", arg)
    }
}
