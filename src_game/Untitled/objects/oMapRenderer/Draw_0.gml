if (room != rm_game) exit;

draw_map(global.map, map_x, map_y, global.map_radius * map_scale);

draw_events_circle(map_x, map_y, global.map_radius * map_scale);

draw_characters_circle(map_x, map_y, global.map_radius * map_scale, map_scale);

draw_active_events_list(40, room_height - 500);