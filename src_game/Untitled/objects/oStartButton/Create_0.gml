// Base scaling
base_scale  = image_xscale;
hover_scale = base_scale * 1.15;
scale       = base_scale;

// Hover interpolation
hover_amount = 0;
hover_speed  = 0.15;

// Glow & bounce state
hovering_last = false;
bounce_timer  = 0;
glow_timer    = 0;

// Colours
base_col  = c_white;
hover_col = make_color_rgb(255, 255, 220);
