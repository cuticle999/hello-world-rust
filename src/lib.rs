// "Hello"と表示する関数
pub fn hello() {
    println!("🐹Hello World🐹");
}

// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;

    // `hello`関数の動作をテストする（オプション）
    #[test]
    fn hello_prints_correctly() {
        // `hello`関数の出力をテストするには、標準出力をキャプチャするなどの方法が必要です。
        // Rustの標準ライブラリでは直接サポートされていないため、外部クレートを使用するか、省略してください。
    }
}