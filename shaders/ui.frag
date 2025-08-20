#version 450
#pragma shader_stage(fragment)
#pragma optimize(on)

struct WidgetData {
    vec4 solidColor;
    vec2 resolution;
    vec2 boundsMin;
    vec2 boundsMax;
    bool isFontTexture;
    float opacity;
};

layout(location = 0) out vec4 fColor;

layout(set = 2, binding = 0) uniform sampler2D sTexture;
layout(set = 3, binding = 0) uniform FyroxData { WidgetData widget; };

layout(location = 0) in vec2 UV;
layout(location = 1) in vec4 Color;


void main() {
    vec4 color = widget.solidColor;

    if (widget.isFontTexture) {
        // Font atlas is single channel (A8), use alpha
        float alpha = texture(sTexture, UV.st).a;
        color = vec4(color.rgb, color.a * alpha);
    } 
    else {
        // Regular textured elements
        color *= texture(sTexture, UV.st);
    }

    color.a *= widget.opacity;
    fColor = color * Color; // modulate with vertex color (selection, etc.)
}

// void main() {
//     fColor = Color * texture(sTexture, UV.st).a;
// }