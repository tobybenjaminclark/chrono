/// --- Ledger Button (Envelope) Draw ---

if (!visible) exit;

/// --- 1. Animated glow behind envelope ---
if (state != "expanded")
{
    var glow_spin_speed   = 0.03;
    var glow_pulse_speed  = 0.04;
    var glow_pulse_amount = 0.2;

    appear_timer += wiggle_speed;

    var glow_angle = appear_timer * glow_spin_speed * 180 / pi;
    var glow_scale = 1 + sin(appear_timer * glow_pulse_speed * 2) * glow_pulse_amount;

    // fade in with envelope scale (smooth 0 → 0.5)
    var glow_alpha = clamp((image_xscale - 0.1) / 0.25, 0, 0.5);

    draw_sprite_ext(
        sprEnvelopeGlow,
        0,
        x,
        y,
        glow_scale * image_xscale * 2.0,
        glow_scale * image_yscale * 2.0,
        glow_angle,
        make_color_rgb(255, 235, 200),
        glow_alpha
    );
	
	/// --- 2. Envelope sprite itself ---
	draw_sprite_ext(sprite_index, 0, x, y, image_xscale, image_yscale, image_angle, c_white, 1);
}




/// --- 3. Instruction text (only when button state) ---
if (state == "button")
{
    var text_off_x = -160;
    var text_off_y = -10;

    draw_set_halign(fa_right);
    draw_set_valign(fa_top);

    // Title
    draw_set_font(fnt_title);
    draw_set_color(c_white);
    draw_text(x + text_off_x, y + text_off_y, "You got some ledgers!");

    // Subtext
    draw_set_font(fnt_troll);
    draw_set_color(make_color_rgb(220, 220, 220));
    draw_text(x + text_off_x, y + text_off_y + 36, "Click this to make some decisions");
}

/// --- 4. Expanded panel ---
if (state == "expanded")
{
    var amt = expand_amount; // 0→1
    var panel_w = 800 * amt;
    var panel_h = 280 * amt;
    var panel_x = room_width - panel_w - 40;  // slides out from envelope edge
    var panel_y = room_height - panel_h - 40;

    // background panel
    draw_set_color(make_color_rgb(30, 30, 45));
    draw_roundrect(panel_x, panel_y, panel_x + panel_w, panel_y + panel_h, false);

    // draw ledger contents
    var ldger = array_length(global.ledgers) > 0 ? global.ledgers[0] : undefined;
    if (ldger != undefined)
    {
        draw_set_halign(fa_left);
        draw_set_valign(fa_top);

        // name
        draw_set_font(fnt_title);
        draw_set_color(c_white);
        draw_text(panel_x + 30, panel_y + 30, ldger.name);

        // description (wrapped)
        draw_set_font(fnt_troll);
        draw_set_color(make_color_rgb(210, 210, 210));
        draw_text_ext(panel_x + 30, panel_y + 80, string(ldger.desc), 20, panel_w * 0.9);
    }

    // small glow accent on edge
    draw_set_color(make_color_rgb(255, 255, 220));
    draw_rectangle(panel_x - 4, panel_y, panel_x, panel_y + panel_h, false);
}
