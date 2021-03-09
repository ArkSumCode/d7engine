#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

out VS_OUTPUT {
    vec2 TexCoord;
    vec4 Color;
} OUT;

uniform float delta;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.TexCoord = TexCoord;
    OUT.Color = vec4(sin(delta), 0.0, 0.0, 0.4);
}
