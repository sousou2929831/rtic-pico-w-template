//! このビルドスクリプトは、クレートルートにある `memory.x` ファイルを、
//! リンカがビルド時に常に見つけられるディレクトリにコピーします。
//!
//! 多くのプロジェクトでは、これは省略可能です。というのも、リンカは常に
//! プロジェクトのルートディレクトリ（`Cargo.toml` がある場所）を検索するからです。
//! しかし、ワークスペースを使用していたり、より複雑なビルド構成をしている場合は、
//! このビルドスクリプトが必要になります。
//!
//! さらに、`memory.x` が変更された際に Cargo にビルドスクリプトを再実行させるよう
//! 指定することで、`memory.x` を更新した際に新しいメモリ設定でアプリケーションが
//! 再ビルドされることが保証されます。

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// (注意) build.rsで利用されるprintln!はただのコンソール出力ではなく、
// Cargo にビルドの設定や依存情報を伝えるための命令として用いられる。
fn main() {
    // `memory.x`を出力ディレクトリに配置しリンカの検索パスに含まれるようにします。
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    // Rustに「このディレクトリからリンカスクリプトを探せ」と指示
    println!("cargo:rustc-link-search={}", out.display());

    // memory.x が変更されたら、次回ビルド時に build.rs を再実行せよ
    // （デフォルトではプロジェクト内のどのファイル変更でもbuild.rsが実行される）
    println!("cargo:rerun-if-changed=memory.x");
}
