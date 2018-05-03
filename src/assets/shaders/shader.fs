#version 330 core
out vec4 FragColor;

in vec3 defaultColor;
in vec2 TexCoord;

uniform sampler2D ourTexture;

void main() {
  FragColor = texture(ourTexture, TexCoord);
  FragColor = vec4(defaultColor, 1.0);
}
