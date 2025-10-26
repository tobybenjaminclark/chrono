/// @desc City Carousel – DRAW

draw_set_font(fnt_troll);

var txt = cities[city_index];
var tw = string_width(txt);
var th = string_height(txt);
var label_w = tw + pad_x * 2;
var label_h = th + pad_y * 2;

// Layout (centered)
var label_x = center_x - label_w * 0.5;
var label_y = center_y - label_h * 0.5;

// Arrows rects (recompute to match STEP)
var left_x1  = label_x - gap - arrow_w;
var left_y1  = center_y - arrow_h * 0.5;
var left_x2  = left_x1 + arrow_w;
var left_y2  = left_y1 + arrow_h;

var right_x1 = label_x + label_w + gap;
var right_y1 = left_y1;
var right_x2 = right_x1 + arrow_w;
var right_y2 = left_y2;

// Panel shadow
draw_set_color(col_bg);
draw_roundrect(label_x + 2, label_y + 2, label_x + label_w + 2, label_y + label_h + 2, false);

// Panel
draw_set_color(col_panel);
draw_roundrect(label_x, label_y, label_x + label_w, label_y + label_h, false);

// Outline
draw_set_color(col_outline);
draw_roundrect(label_x, label_y, label_x + label_w, label_y + label_h, true);

// City text
draw_set_halign(fa_center);
draw_set_valign(fa_middle);
draw_set_color(col_text);
draw_text(label_x + label_w * 0.5, label_y + label_h * 0.5, txt);

// ----- Left Arrow -----
var a_col = hover_left ? col_hover : col_arrow;
draw_set_color(a_col);
// Draw a simple left-pointing triangle centered in the rect
var cx = (left_x1 + left_x2) * 0.5;
var cy = (left_y1 + left_y2) * 0.5;
var w2 = (left_x2 - left_x1) * 0.35;
var h2 = (left_y2 - left_y1) * 0.35;
draw_triangle(cx - w2, cy, cx + w2, cy - h2, cx + w2, cy + h2, false);

// Arrow outline
draw_set_color(col_outline);
draw_rectangle(left_x1, left_y1, left_x2, left_y2, true);

// ----- Right Arrow -----
a_col = hover_right ? col_hover : col_arrow;
draw_set_color(a_col);
// Right-pointing triangle
cx = (right_x1 + right_x2) * 0.5;
cy = (right_y1 + right_y2) * 0.5;
w2 = (right_x2 - right_x1) * 0.35;
h2 = (right_y2 - right_y1) * 0.35;
draw_triangle(cx + w2, cy, cx - w2, cy - h2, cx - w2, cy + h2, false);

// Arrow outline
draw_set_color(col_outline);
draw_rectangle(right_x1, right_y1, right_x2, right_y2, true);

// Restore defaults
draw_set_halign(fa_left);
draw_set_valign(fa_top);
draw_set_color(c_white);
