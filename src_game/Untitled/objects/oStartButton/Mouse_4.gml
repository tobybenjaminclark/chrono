
audio_play_sound(healingspell, 0, false, 0.5);

fade_to_room(rm_game);

var city = oCitySelector.selected_city;
show_debug_message("Current city: " + city);
init_map(city)