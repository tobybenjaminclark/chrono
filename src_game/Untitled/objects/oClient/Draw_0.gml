var col = global.is_connected ? c_lime : c_red;

// Draw shadow/background
draw_set_color(c_black);
draw_circle(x + 1, y + 1, radius, false);

// Draw indicator
draw_set_color(col);
draw_circle(x, y, radius, false);