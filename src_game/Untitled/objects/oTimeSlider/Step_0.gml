if (keyboard_check(vk_right)) t += speed;
if (keyboard_check(vk_left))  t -= speed;

// Clamp so it stops at the edges
t = clamp(t, 0, 1);
