use ooze::render::*;

fn main() {
    let (e, d) = init_window(100, 100, "test");

    let mut tr = Terminal::new(d.clone(), 3, 3, 2, 32, 32);
    let mut tl = TextureLibrary::new(d.clone());
    tl.build_string("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), "./resources/fonts/Roboto-Regular.ttf", (32, 32));

    let program = default_program(&d.clone());

    tr.set(2, 2, 1, Glyph::new(tl.get('a'), &program));
    tr.set(0, 0, 1, Glyph::new(tl.get('a'), &program));
    tr.render();
}