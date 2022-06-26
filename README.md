# reijou

## これはなんですの？

このクレートは[セバスチャンマクロを作って学ぶRustの手続きマクロ](https://zenn.dev/kazatsuyu/articles/33e130563b87b1)という記事のために作られたライブラリですわ。

`セバスチャン`マクロを使って以下のようにお嬢様らしくプログラミングができますわ。

```rust
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
```

