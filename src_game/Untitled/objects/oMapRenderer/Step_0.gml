
// Smoothly animate position and scale
map_y = lerp(map_y, map_target_y, map_lerp_speed);
map_scale = lerp(map_scale, map_target_scale, map_lerp_speed);

if (keyboard_check_pressed(vk_space)) {
    // swap between zoomed-in and zoomed-out
    if (map_target_scale == 1) {
        map_target_scale = 1/5;
        map_target_y = room_height / 4;
    } else {
        map_target_scale = 1;
        map_target_y = room_height / 2;
    }
}
