// --- Fade timing ---
lifetime_total = 3 * room_speed;   // total = 3 seconds
fade_delay     = 2 * room_speed;   // stay fully visible for 2 seconds
fade_duration  = 1 * room_speed;   // fade out over 1 second

// --- Start fully visible ---
image_alpha = 1;
timer = 0;
