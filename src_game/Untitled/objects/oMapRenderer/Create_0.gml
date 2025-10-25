if !variable_global_exists("map") { init_default_map(); }
if !variable_global_exists("events") { init_default_events(); }
if array_length(global.events) == 0 { init_default_events(); }


// oMapController: Create Event
map_x = room_width / 2;          // always centered horizontally
map_y = room_height / 2;         // starts middle
map_scale = 1;                   // normal size

// targets (for animation)
map_target_y = room_height / 3;  // top third
map_target_scale = 1/3;          // one third size

// interpolation speed (0.1 = smooth but responsive)
map_lerp_speed = 0.1;
