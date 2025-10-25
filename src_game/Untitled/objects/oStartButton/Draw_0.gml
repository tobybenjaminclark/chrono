// Lighten colour when hovered
var mix_col = merge_color(base_col, hover_col, glow_timer);

// Subtle additive glow
if (glow_timer > 0) {
    var alpha = 0.3 * glow_timer;
    draw_set_color(make_color_rgb(255, 255, 180));
    draw_sprite_ext(sprite_index, image_index, x, y, scale * 1.1, scale * 1.1, 0, c_white, alpha);
}

// Draw main sprite
draw_set_color(mix_col);
draw_sprite_ext(sprite_index, image_index, x, y, scale, scale, 0, mix_col, 1);
draw_set_color(c_white);
