/// Draw Event
if (room != rm_game) exit;

var t = global.VIEWTIME;

// --- Map t ∈ [0,1] to an angle on the time circle ---
var angle = 135 + (270 * t);
var rad   = degtorad(angle);
var px    = center_x + radius * cos(rad);
var py    = center_y + radius * sin(rad);

// --- Draw knob ---
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

// Interpolate color for t
var col_start = make_color_rgb(100, 150, 255);
var col_end   = make_color_rgb(255, 220, 120);
var col       = merge_color(col_start, col_end, clamp(t, 0, 1));

// --- Draw event counter ---
draw_set_color(c_white);
var event_text = string(array_length(global.events)) + "x Events";
draw_text(label_x, label_y - 30, event_text);

// --- Arrow setup ---
var text_w = string_width(event_text);
var text_h = string_height(event_text);
var arrow_offset = 50;
var arrow_size   = 18;

// Arrow coordinates (still in room space)
var left_x  = label_x - text_w / 2 - arrow_offset;
var right_x = label_x + text_w / 2 + arrow_offset;
var arrow_y = label_y - 30 + text_h / 2;

// --- Hover detection (room coords) ---
var mx = mouse_x;
var my = mouse_y;
var hover_box = 24;
var hover_left  = point_in_rectangle(mx, my, left_x - hover_box,  arrow_y - hover_box,
                                             left_x + hover_box,  arrow_y + hover_box);
var hover_right = point_in_rectangle(mx, my, right_x - hover_box, arrow_y - hover_box,
                                             right_x + hover_box, arrow_y + hover_box);

// --- Animation helper ---
if (!variable_global_exists("arrow_timer")) global.arrow_timer = 0;
global.arrow_timer += 0.15;
var pulse = 0.5 + 0.5 * sin(global.arrow_timer);

// --- Draw arrow triangles ---
draw_set_alpha(1);
draw_set_color(hover_left ? merge_color(c_yellow, c_white, pulse) : c_ltgray);
draw_triangle(
    left_x + arrow_size,  arrow_y - arrow_size,
    left_x + arrow_size,  arrow_y + arrow_size,
    left_x - arrow_size,  arrow_y,
    false
);

draw_set_color(hover_right ? merge_color(c_yellow, c_white, pulse) : c_ltgray);
draw_triangle(
    right_x - arrow_size, arrow_y - arrow_size,
    right_x - arrow_size, arrow_y + arrow_size,
    right_x + arrow_size, arrow_y,
    false
);

// --- Activate on hover ---
var hover_speed = speed;
if (hover_left)  t -= hover_speed;
if (hover_right) t += hover_speed;

// Clamp and apply
t = clamp(t, 0, 1);
global.VIEWTIME = t;

// --- Stylised label: "-- 0.32 --"
draw_set_color(col);
var label_text = "-- " + string_format(t, 0, 2) + " --";
draw_text(label_x, label_y - 5, label_text);

// --- Controls info ---
draw_set_color(c_white);
draw_set_font(fnt_troll);
draw_text(label_x, label_y + 20, "Hover arrows to move through time");
