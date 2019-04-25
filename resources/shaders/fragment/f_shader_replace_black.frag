#version 140

in vec2 v_tex_coords;

out vec4 color;

uniform vec4 bg_color;
uniform vec4 fg_color;
uniform sampler2D tex;

void main() {
    if (texture(tex, v_tex_coords).a == 0.0) {
        color = bg_color;
    } else {
        vec4 t_color = texture(tex, v_tex_coords);
        color = fg_color * vec4(1.0 - t_color.rgb, t_color.a);
    }
}