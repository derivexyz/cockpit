pub mod types;

// auto-generates a module called types using stubs from the types directory
use automod::dir;

pub mod generated {
    automod::dir!(pub "./src/generated");
}
