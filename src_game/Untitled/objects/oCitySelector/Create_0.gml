/// @desc City Carousel – CREATE
cities = [
    "Nottingham","Birmingham","Manchester","Glasgow","Leeds",
    "Liverpool","Edinburgh","Bristol","Sheffield","London",
    "Leicester","Cardiff","Belfast","Newcastle upon Tyne","Coventry",
    "Brighton","Southampton","Cambridge","Oxford","York"
];

city_index = 0;
selected_city = cities[city_index];

center_x = x;
center_y = y;

// Sizing
pad_x  = 24;   // horizontal padding around label
pad_y  = 12;   // vertical padding around label
arrow_w = 28;  // arrow button width
arrow_h = 28;  // arrow button height
gap     = 10;  // gap between label and arrows

// Style
col_bg      = make_color_rgb(20, 24, 32);
col_panel   = make_color_rgb(36, 42, 56);
col_text    = c_white;
col_arrow   = make_color_rgb(220, 220, 240);
col_hover   = make_color_rgb(255, 240, 200);
col_outline = make_color_rgb(80, 90, 110);

hover_left  = false;
hover_right = false;

// Optional: set a font if you have one
// draw_set_font(fnt_title);
