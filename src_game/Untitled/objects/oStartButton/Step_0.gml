// Hover detection
var hovering = point_in_rectangle(
    mouse_x, mouse_y,
    x - sprite_width  * scale / 2,
    y - sprite_height * scale / 2,
    x + sprite_width  * scale / 2,
    y + sprite_height * scale / 2
);

// Smooth interpolation of hover amount
hover_amount = lerp(hover_amount, hovering, hover_speed);

// When mouse *just* enters, trigger a quick bounce
if (hovering && !hovering_last) {
    bounce_timer = 0.0;
}
hovering_last = hovering;

// Advance bounce timer if active
if (hovering) {
    bounce_timer = min(bounce_timer + 0.15, 1);
} else {
    bounce_timer = max(bounce_timer - 0.1, 0);
}

// Compute a small overshoot curve for the bounce
var bounce = 0.04 * sin(bounce_timer * pi);

// Scale up smoothly (with small bounce)
scale = lerp(base_scale, hover_scale + bounce, hover_amount);

// Soft glowing timer for pulse brightness
glow_timer += (hovering ? 0.1 : -0.05);
glow_timer = clamp(glow_timer, 0, 1);

// Detect hover state (reuse your existing hover check)
var hovering = point_in_rectangle(
    mouse_x, mouse_y,
    x - sprite_width  * scale / 2,
    y - sprite_height * scale / 2,
    x + sprite_width  * scale / 2,
    y + sprite_height * scale / 2
);

// Tell everyone whether the start button is hovered
global.start_hovered = hovering;
