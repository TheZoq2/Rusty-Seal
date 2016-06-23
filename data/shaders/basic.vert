
#version 140
layout(std140) uniform;


in vec3 position;
in vec3 normal;

uniform mat3 transform;

//Position and normal in the world of the vertex
out vec4 vertex_position;
out vec3 vertex_normal;

vec4 pos;

void main()
{
    mat4 modelViewMatrix = view_matrix * modelMatrix;
    vertex_position = modelViewMatrix * (vec4(position, 1.0));
    pos = projection_matrix * vertex_position;

    vertex_normal = transpose(inverse(mat3(modelMatrix))) * normal;

    vertex_color = (position.xyz + vec3(1, 1, 1)) / 2;

    gl_Position = pos;
}


