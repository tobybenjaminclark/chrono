if (keyboard_check(vk_right)) t += speed;
if (keyboard_check(vk_left))  t -= speed;

// Clamp so it stops at the edges
t = clamp(t, 0, 1);
global.VIEWTIME = t;

var r = global.map_radius;
var scx = 0;
var scy = 0;
with (oMapRenderer) {
	r = map_scale * global.map_radius;
	scx = map_x;
	scy = map_y;
}
center_x = scx;
center_y = scy
radius = r;

/// Step Event
if (room != rm_game) exit;


// Use same geometry as in Draw
var label_y = center_y + radius - 10;
var label_x = center_x;
var event_text = string(array_length(global.events)) + "x Events";

var text_w = string_width(event_text);
var text_h = string_height(event_text);
var arrow_offset = 50;
var arrow_size   = 18;

var left_x  = label_x - text_w / 2 - arrow_offset;
var right_x = label_x + text_w / 2 + arrow_offset;
var arrow_y = label_y - 30 + text_h / 2;

// --- Mouse position (room coordinates) ---
var mx = mouse_x;
var my = mouse_y;
var hover_box = 24;

// --- Hover state detection ---
hover_left  = point_in_rectangle(mx, my, left_x - hover_box,  arrow_y - hover_box,
                                           left_x + hover_box,  arrow_y + hover_box);
hover_right = point_in_rectangle(mx, my, right_x - hover_box, arrow_y - hover_box,
                                           right_x + hover_box, arrow_y + hover_box);

// --- Animation timer (for pulse) ---
if (!variable_global_exists("arrow_timer")) global.arrow_timer = 0;
global.arrow_timer += 0.15;

// --- Time control ---
var hover_speed = speed / 2; // reuse from instance variable
if (hover_left)  t -= hover_speed;
if (hover_right) t += hover_speed;

// --- Clamp and apply ---
t = clamp(t, 0, 1);
global.VIEWTIME = t;
