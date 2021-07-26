extern crate cc;

fn main() {
    cc::Build::new()
        .file("native/doubler.c")
        .compile("libdoubler.a");

    cc::Build::new()
        .cpp(true)
        .file("native-cpp/doubler.cpp")
        .compile("libdoublercpp.a");
}