fn main() {
    cc::Build::new()
        .file("jos_foo/jos_foo.c")
        .include("jos_foo")
        .compile("jos_foo");
}
