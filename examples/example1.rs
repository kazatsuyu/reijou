use reijou::セバスチャン;

セバスチャン! {
    わたくし std::env::args 様を使わせていただきますわ.
}

セバスチャン! {
    わたくし std::io::Write 様を使わせていただきますわ.
}

セバスチャン! {
    こちらの f 様は,
    a: i32 と b: &str をお受け取りになって,
    std::io::Result<()> をお返しになり,
    以下のことをなさいますのよ. {
        writeln!(std::io::stdout(), "a: {}", a)?;
        writeln!(std::io::stdout(), "b: {}", b)?;
        Ok(())
    }
}

fn main() {
    f(42, &args().next().unwrap()).unwrap();
}
