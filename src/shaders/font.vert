#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec3 Color;

out VS_OUTPUT {
    vec2 TexCoord;
    vec3 Color;
} OUT;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.TexCoord = TexCoord;
    OUT.Color = Color;
}
