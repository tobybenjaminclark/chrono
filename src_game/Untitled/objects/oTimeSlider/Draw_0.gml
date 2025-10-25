var t = global.VIEWTIME;

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
draw_set_font(fnt_title);

var label_y = center_y + radius - 10;
var label_x = center_x;

draw_set_halign(fa_center);
draw_set_valign(fa_top);

// Interpolate between two colors as VIEWTIME changes
var t = clamp(global.VIEWTIME, 0, 1);
var col_start = make_color_rgb(100, 150, 255); // cool blue
var col_end   = make_color_rgb(255, 220, 120); // warm gold
var col = merge_color(col_start, col_end, t);

// Draw stylised label: "-- 0.32 --"
draw_set_color(col);
var label_text = "-- -- " + string_format(t, 0, 2) + " -- --";
draw_text(label_x, label_y, label_text);

// Restore defaults
draw_set_color(c_white);
draw_set_font(fnt_troll);