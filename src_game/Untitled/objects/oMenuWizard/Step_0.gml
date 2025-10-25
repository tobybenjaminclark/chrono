// Read hover state from global
var target = (global.start_hovered) ? hover_scale : base_scale;

// Smoothly scale toward target
scale = lerp(scale, target, hover_speed);
