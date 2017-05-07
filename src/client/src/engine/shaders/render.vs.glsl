#version 410

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 model_pos;

void main() {
    v_normal = transpose(inverse(mat3(model_pos))) * normal;
    gl_Position = model_pos * vec4(position, 1.0);
}