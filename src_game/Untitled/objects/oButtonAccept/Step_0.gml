/// STEP EVENT

// --- Fade-in ---
if (fade_alpha < 1) {
    fade_alpha = min(fade_alpha + fade_speed, 1);
}


// --- Hover detection ---
hovering = point_in_rectangle(
    mouse_x, mouse_y,
    x - sprite_width  * scale / 2,
    y - sprite_height * scale / 2,
    x + sprite_width  * scale / 2,
    y + sprite_height * scale / 2
);

// --- Smooth hover blend ---
hover_amount = lerp(hover_amount, hovering, hover_speed);

// --- Bounce control ---
if (hovering && !hovering_last) {
    bounce_timer = 0.0; // reset when entering hover
}
hovering_last = hovering;

// Only advance bounce while hovering (so it completes once)
if (hovering) {
    bounce_timer = min(bounce_timer + 0.2, 1);
}

// --- Compute bounce factor ---
var bounce = 0;
if (hovering) {
    // Stronger at the start of hover, fades out
    bounce = 0.05 * sin(bounce_timer * pi);
}

// --- Combine scale smoothly ---
scale = base_scale + (hover_amount * ((hover_scale - base_scale) + bounce));

// --- Glow pulse ---
glow_timer += (hovering ? 0.1 : -0.05);
glow_timer = clamp(glow_timer, 0, 1);

// --- Click detection ---
if (hovering && mouse_check_button_pressed(mb_left)) {
	audio_play_sound(accept_snd, 0, false, 0.5);
	accept_ledger();
	
	var ledge = global.ledgers[0];
	if ledge.sat == false {
		fade_to_room(rm_lose);
	}
	
    if (instance_exists(ledger_parent)) ledger_parent.accept();
}
