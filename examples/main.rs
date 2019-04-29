use ooze::render::*;

fn main() {
    let (e, d) = init_window(1024, 512, "test");

    let mut tr = Terminal::new(d.clone(), 64, 32, 2);
    let mut tl = TextureLibrary::new(d.clone(), "./resources/fonts/Perfect DOS VGA 437.ttf", (16, 16));
    tl.build_string("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned());

    let program = build_program(&d.clone(), "./resources/shaders/vertex/v_shader_default.vert", "./resources/shaders/fragment/f_shader_replace_black.frag");

    tr.set(0, 1, 1, Glyph::new(tl.get('b'), &program));
    tr.set(0, 0, 1, Glyph::new(tl.get('O'), &program));
    tr.set(1, 0, 1, Glyph::new(tl.get('c'), &program));
    tr.set(1, 1, 1, Glyph::new(tl.get('d'), &program));
    tr.set(2, 2, 1, Glyph::new(tl.get('Z'), &program));
    tr.fill_with(0, Glyph::new(tl.get('X'), &program));
    loop {
        tr.render();
    }
}