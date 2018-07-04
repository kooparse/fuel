#version 330 core
out vec4 FragColor;

void main() {
  vec3 objectColor = vec3(1.0f, 0.5f, 0.31f);
  vec3 lightColor = vec3(1.0f, 1.0f, 1.0f);
  FragColor = vec4(lightColor * objectColor, 1.0);
}
