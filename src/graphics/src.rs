pub const VS: &'static str = r#"
#version 330 core
in vec2 a_pos;
in vec4 a_color;

out vec4 v_color;

uniform vec2 u_offset;

void main() {
    gl_Position = vec4(a_pos + u_offset, 1.0, 1.0);
    v_color = a_color;
}
"#;

pub const VS_ATTR_POS: &'static str = "a_pos";
pub const UNIFORM_OFFSET: &'static str = "u_offset";

pub const FS: &'static str = r#"
#version 330 core
in vec4 v_color;

out vec4 fragColor;

void main() {
    fragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;