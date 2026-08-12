#version 330 core
in vec2 v_tex_coords;

out vec4 color;

const float outlineThickness = 1.0;
const vec4 outlineColor = vec4(0.0,0.0,0.0,1.0);

// `tex` is expected to be your existing LOW-RESOLUTION render target — the
// one you already produce by rendering the scene at reduced resolution.
// Pixelation itself is NOT done in this shader anymore; it's already baked
// into `tex` by the time this runs. This shader's job is just to sample it
// (with nearest filtering) and dither it while upscaling to screen size.
uniform sampler2D tex;

// Size of the LOW-RES texture in texels (e.g. if you render your scene at
// 320x180 before upscaling to a 1920x1080 screen, this is (320.0, 180.0) —
// NOT the screen resolution).
uniform vec2 u_sourceResolution = vec2(640, 360);

uniform float u_levels = 8.0;      // color levels per channel, e.g. 4.0 or 8.0
uniform float u_ditherAmt = 0.8;   // 0.0 = no dither, 1.0 = full strength

// --- Recursive Bayer dithering ---------------------------------------------
// Builds Bayer-matrix threshold values purely from floor/fract math, so there
// is no array indexing and no "const index" compiler error. Bayer2 is the
// base case; Bayer4/Bayer8 recurse on it to build bigger matrices.
float Bayer2(vec2 a) {
    a = floor(a);
    return fract(a.x / 2.0 + a.y * a.y * 0.75);
}
#define Bayer4(a) (Bayer2(0.5 * (a)) * 0.25 + Bayer2(a))
#define Bayer8(a) (Bayer4(0.5 * (a)) * 0.25 + Bayer2(a))
// -----------------------------------------------------------------------------

void main() {
    // Sample the already-pixelated low-res texture directly. As long as
    // `tex` has GL_NEAREST filtering set on the host side, every fragment
    // within a given source texel reads back exactly the same flat color —
    // no averaging/interpolation artifacts, since the real downsampling
    // already happened when `tex` was rendered.
    vec4 sampled = texture(tex, v_tex_coords);
    vec3 baseColor = sampled.rgb;

    // ORDERED DITHERING: use the LOW-RES texel grid (not the screen's native
    // resolution) as the dither cell, so the dither pattern lines up exactly
    // with the visible pixel blocks rather than looking noisy/misaligned.
    vec2 cell = floor(v_tex_coords * u_sourceResolution);
    float threshold = Bayer4(cell) - 0.5; // swap for Bayer4(cell) for a coarser look

    float levelStep = 1.0 / (u_levels - 1.0);
    baseColor += threshold * levelStep * u_ditherAmt;

    // QUANTIZE to u_levels color levels per channel, for that limited-
    // palette retro look. Set u_levels high (e.g. 32+) if you just want
    // dithering without visible color banding.
    baseColor = floor(baseColor * (u_levels - 1.0) + 0.5) / (u_levels - 1.0);
    baseColor = clamp(baseColor, 0.0, 1.0);

    color = vec4(baseColor, sampled.a);
}
