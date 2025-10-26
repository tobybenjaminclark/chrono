// Card background
var col = hover ? make_color_rgb(255, 230, 180) : make_color_rgb(240, 240, 240);
draw_set_color(col);
draw_rectangle(x - 70, y - 100, x + 70, y + 100, false);

// Title
draw_set_font(fnt_title);
draw_set_color(c_black);
draw_set_halign(fa_center);
draw_text(x, y - 60, ledger_data.name);

// Description
draw_set_font(fnt_troll);
draw_set_color(c_black);
draw_text(x, y - 30, ledger_data.desc);
