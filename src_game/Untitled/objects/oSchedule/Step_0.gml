// --- Advance timer ---
timer += 1;

// --- Start fading after delay ---
if (timer > fade_delay) {
    var fade_t = (timer - fade_delay) / fade_duration; // 0 → 1
    image_alpha = 1 - fade_t;
}

// --- Destroy when finished ---
if (timer >= lifetime_total) {
    instance_destroy();
}
