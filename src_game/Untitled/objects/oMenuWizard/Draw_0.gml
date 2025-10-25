// Determine dimming factor based on how small he is (relative to hover_scale)
var t = clamp((scale - base_scale) / (hover_scale - base_scale), 0, 1);

// Blend from dark blue → normal bright white
var dark_col = make_color_rgb(120, 160, 255); // cool bluish tone for background
var bright_col = c_white;

// Colour depends on scale
var draw_col = merge_color(dark_col, bright_col, t);

// Optional glow when hovered
if (global.start_hovered) {
    draw_set_color(make_color_rgb(255, 240, 200));
    draw_sprite_ext(sprite_index, image_index, x, y, scale * 1.1, scale * 1.1, 0, c_white, 0.1);
}

// Main sprite (dimmed/brightened)
draw_set_color(draw_col);
draw_sprite_ext(sprite_index, image_index, x, y, scale, scale, 0, draw_col, 1);
draw_set_color(c_white);
