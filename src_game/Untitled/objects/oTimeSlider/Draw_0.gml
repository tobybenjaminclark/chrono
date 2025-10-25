// Map t ∈ [0,1] to an angle in degrees
// 0 → bottom-left (135°)
// 1 → bottom-right (405°) — moves counter-clockwise over the top
var angle = 135 + (270 * t);
var rad   = degtorad(angle);

// Compute position on circle
var px = center_x + radius * cos(rad);
var py = center_y + radius * sin(rad);

// --- Draw the knob ---
draw_set_color(c_white);
draw_circle(px, py, 5, false);
draw_set_color(c_aqua);
draw_circle(px, py, 3, false);

// --- Draw t value UNDER the map ---
var label_y = center_y + radius + 15;  // 30px below the circle
var label_x = center_x;                // centered horizontally
draw_set_halign(fa_center);
draw_set_valign(fa_top);
draw_set_color(c_white);
draw_text(label_x, label_y, string_format(global.VIEWTIME, 0, 2));
