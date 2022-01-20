#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

uniform mat4 projection;

out VS_OUTPUT {
    vec2 TexCoord;
} OUT;

void main()
{
    gl_Position = projection * vec4(Position, 1.0);
    OUT.TexCoord = TexCoord;
}
