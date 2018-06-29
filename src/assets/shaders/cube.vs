#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTexCoord0;
layout(location = 1) in vec2 aTexCoord1;

out vec2 TexCoord;
uniform mat4 transform;
uniform mat4 mvp;

void main() {
  gl_Position = mvp * vec4(aPos, 1.0f);
  # TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}
