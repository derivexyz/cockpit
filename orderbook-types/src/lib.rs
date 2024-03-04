// auto-generates a module called types using stubs from the types directory
use automod::dir;

pub mod types {
    automod::dir!(pub "./src/types");
}
