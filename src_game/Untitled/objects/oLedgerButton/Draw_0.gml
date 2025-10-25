
if (visible)
{
    // --- Animate glow ---
    var glow_spin_speed   = 0.1;   // slow rotation
    var glow_pulse_speed  = 0.05;   // pulse rate
    var glow_pulse_amount = 0.1;    // ±10% pulse

    appear_timer += wiggle_speed; // move timer up here (drives both animations)

    var glow_angle = appear_timer * glow_spin_speed * 180 / pi; // convert to degrees
    var glow_scale = 1 + sin(appear_timer * glow_pulse_speed * 2) * glow_pulse_amount;

    // --- Fade in opacity smoothly with the envelope scale ---
    var glow_alpha = clamp((image_xscale - 0.2) / 0.05, 0, 0.3); // 0→0.6 fade

    // --- Draw glow behind envelope (additive blend) ---
    draw_sprite_ext(
        sprEnvelopeGlow,
        0,
        x,
        y,
        glow_scale * 0.45, // slightly larger
        glow_scale * 0.45,
        glow_angle,
        make_color_rgb(255, 240, 200),
        glow_alpha
    );

    // --- Slide & zoom in ---
    if (x > x_target) {
        x = lerp(x, x_target, 0.2);
    }

    // --- Wiggle between 0.25 ↔ 0.30 ---
    var base_scale = 0.275;
    var range = 0.025;
    var scale_variation = base_scale + sin(appear_timer * 2) * range;

    image_xscale = scale_variation;
    image_yscale = scale_variation;

    // --- Optional small rotation wiggle ---
    image_angle = sin(appear_timer) * wiggle_amount;

    // --- Draw envelope itself last ---
    draw_self();
}
if (visible)
{
    // --- Draw text to the left of the envelope ---
    var text_offset_x = -160; // how far left of the icon
    var text_offset_y = -10;  // vertical alignment tweak

    // Title text
    draw_set_font(fnt_title);
    draw_set_halign(fa_right);
    draw_set_valign(fa_top);
    draw_set_color(c_white);
    draw_text(x + text_offset_x, y + text_offset_y, "You got some ledgers!");

    // Subtext
    draw_set_font(fnt_troll);
    draw_set_halign(fa_right);
    draw_set_valign(fa_top);
    draw_set_color(make_color_rgb(220, 220, 220));
    draw_text(x + text_offset_x, y + text_offset_y + 40, "Click this to make some decisions");
}

else
{
    // --- Reset when hidden ---
    x = x_start;
    image_xscale = 0.2;
    image_yscale = 0.2;
    appear_timer = 0;
    image_angle = 0;
}
