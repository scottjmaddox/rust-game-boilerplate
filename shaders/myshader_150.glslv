#version 150 core

in vec4 a_Pos;
in vec4 a_Color;

uniform Uniforms {
    mat4 u_Transform;
};

out vec4 v_Color;

void main() {
    v_Color = a_Color;
    gl_Position = a_Pos * u_Transform;
}
