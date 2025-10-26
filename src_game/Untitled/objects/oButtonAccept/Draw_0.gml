/// DRAW EVENT

// Compute glow brightness (soft pulse)
var pulse = 0.5 + 0.5 * sin(current_time / 200.0);
var col   = merge_color(base_col, hover_col, glow_timer * pulse);

// --- Draw glowing aura ---
var glow_size = sprite_width * scale * (1.2 + 0.1 * pulse);
draw_set_color(col);
draw_sprite_ext(sprite_index, 0, x, y, glow_size / sprite_width, glow_size / sprite_height, 0, col, 0.4 * fade_alpha);

// --- Draw main sprite ---
draw_sprite_ext(sprite_index, 0, x, y, scale, scale, 0, c_white, fade_alpha);

// --- Label ---
draw_set_halign(fa_center);
draw_set_alpha(fade_alpha);
draw_text(x, y - ((sprite_width * scale) / 2) - 15, "Accept");
draw_set_alpha(1); // restore
draw_set_halign(fa_left);
