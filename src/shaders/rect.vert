#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

uniform float zoom;

void main()
{
    vec3 new_position = vec3(Position.x * zoom, Position.y * zoom, Position.z);

    gl_Position = vec4(new_position, 1.0);
    OUT.Color = Color;
}
