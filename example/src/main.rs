#[derive(serde_derive::Serialize)]
struct _Vec2D<T> {
    x: T,
    y: T,
}

#[derive(serde_derive_next::Serialize)]
struct _Vec2DNext<T> {
    x: T,
    y: T,
}

fn main() {}
