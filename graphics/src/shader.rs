static VertexShaderCode: &str = r#"
#version 330 core

layout(location = 0) in vec2 aPosition;

uniform mat4 project;
uniform mat4 model;
uniform mat4 view;

void main() {
    gl_Position = project * model * view * vec4(aPosition, 0.0, 1.0);
}"#;

static FragShaderCode: &str = r#"
#version 330 core

layout(location = 0) out vec

out vec4 FragColor;

uniform vec4 color;

void main() {
    FragColor = color;
}"#;