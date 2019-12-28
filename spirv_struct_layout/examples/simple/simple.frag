#version 450 core

layout(std430, binding = 0) buffer Uniforms {
  mat4 model_view;
  vec3 light_dir;
  vec4 position;
} buf;

layout(location = 0) out vec4 outColor;

void main() {
    vec4 p = buf.model_view * buf.position;
    float d = dot(p.xyz, buf.light_dir);
    outColor = vec4(vec3(d), 1.0);
}