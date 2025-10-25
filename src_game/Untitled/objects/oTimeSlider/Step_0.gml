if (keyboard_check(vk_right)) t += speed;
if (keyboard_check(vk_left))  t -= speed;

// Clamp so it stops at the edges
t = clamp(t, 0, 1);
global.VIEWTIME = t;

var r = global.map_radius;
var scx = 0;
var scy = 0;
with (oMapRenderer) {
	r = map_scale * global.map_radius;
	scx = map_x;
	scy = map_y;
}
center_x = scx;
center_y = scy
radius = r;