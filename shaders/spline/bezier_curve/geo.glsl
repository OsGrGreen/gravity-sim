#version 450 core

// Input: lines
layout(lines) in;
// Output: triangle strip with up to 4 vertices
layout(triangle_strip, max_vertices = 4) out;

// Uniforms for screen resolution and desired line thickness (in pixels)
uniform vec2 u_screenSize;   // e.g. (width, height)
uniform float u_thickness;   // desired thickness in pixels

// This helper converts a position (given in screen space, in pixels)
// back into clip space using the original gl_Position.w and z values.
vec4 ClipFromScreen(vec2 screenPos, float origW, float origZ) {
    // Convert screen position (pixels) to normalized device coordinates ([-1, 1])
    vec2 ndc = (screenPos / u_screenSize) * 2.0 - 1.0;
    // Reconstruct clip-space position.
    return vec4(ndc * origW, origZ, origW);
}

void main() {
    // Retrieve the two input vertices (in clip space)
    vec4 clipPos0 = gl_in[0].gl_Position;
    vec4 clipPos1 = gl_in[1].gl_Position;
    
    // Convert clip space positions to NDC by perspective division.
    vec2 ndc0 = clipPos0.xy / clipPos0.w;
    vec2 ndc1 = clipPos1.xy / clipPos1.w;
    
    // Convert NDC to screen coordinates (in pixels).
    vec2 screenPos0 = (ndc0 * 0.5 + 0.5) * u_screenSize;
    vec2 screenPos1 = (ndc1 * 0.5 + 0.5) * u_screenSize;
    
    // Compute the line direction in screen space and then a perpendicular vector.
    vec2 dir = normalize(screenPos1 - screenPos0);
    vec2 perp = vec2(-dir.y, dir.x);
    
    // Compute the offset (half the thickness).
    vec2 offset = perp * (u_thickness * 0.5);
    
    // Compute the offset screen positions for both vertices.
    vec2 s0_left  = screenPos0 + offset;
    vec2 s0_right = screenPos0 - offset;
    vec2 s1_left  = screenPos1 + offset;
    vec2 s1_right = screenPos1 - offset;
    
    // Now, emit four vertices as a triangle strip.
    // The ordering below produces two triangles that form the quad.
    
    // Vertex corresponding to the left side of the first vertex.
    gl_Position = ClipFromScreen(s0_left, clipPos0.w, clipPos0.z);
    EmitVertex();
    
    // Vertex corresponding to the left side of the second vertex.
    gl_Position = ClipFromScreen(s1_left, clipPos1.w, clipPos1.z);
    EmitVertex();
    
    // Vertex corresponding to the right side of the first vertex.
    gl_Position = ClipFromScreen(s0_right, clipPos0.w, clipPos0.z);
    EmitVertex();
    
    // Vertex corresponding to the right side of the second vertex.
    gl_Position = ClipFromScreen(s1_right, clipPos1.w, clipPos1.z);
    EmitVertex();
    
    EndPrimitive();
}