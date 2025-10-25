function init_map(location) {
    if (global.client_socket != undefined) {
        var t_buffer = buffer_create(256, buffer_grow, 1);
        buffer_seek(t_buffer, buffer_seek_start, 0);

        var json_string = "{ \"INIT_MAP\": { \"loc_str\": \"" + string(location) + "\" } }";
        buffer_write(t_buffer, buffer_string, json_string);
        network_send_packet(global.client_socket, t_buffer, buffer_tell(t_buffer));
        buffer_delete(t_buffer);
    } else {
        show_message("No active TCP connection!");
    }
}


/// @function handle_init_map(data)
/// @desc Builds global.map from server-sent JSON structure
function handle_init_map(data)
{
    show_debug_message("Handle init map");
    show_debug_message(json_encode(data));

    // --- Validate expected fields ---
    if (!variable_struct_exists(data, "map")) {
        show_debug_message("[handle_init_map] Missing 'map' field.");
        return;
    }

    // Extract sections
    var places_raw  = data.map[0]; // first nested array = places
    var routes_raw  = data.map[1]; // second nested array = routes
    var ownership   = data.ownership;

    // --- Build Place structs ---
    var places = array_create(array_length(places_raw));
    for (var i = 0; i < array_length(places_raw); i++) {
        var p = places_raw[i]; // e.g. ["Nottingham Castle", [-0.17,-0.50]]
        var name = p[0];
        var coords = p[1];
        places[i] = new Place(name, coords[0], coords[1]);
    }

    // --- Build Route arrays ---
    var routes = array_create(array_length(routes_raw));
    for (var i = 0; i < array_length(routes_raw); i++) {
        var route_points = routes_raw[i];
        var new_route = array_create(array_length(route_points));
        for (var j = 0; j < array_length(route_points); j++) {
            var pt = route_points[j];
            new_route[j] = new Vec2(pt[0], pt[1]);
        }
        routes[i] = new_route;
    }

    // --- Construct Map object ---
    global.map = new Map(places, routes);
    global.map.ownership = ownership; // attach ownership table (if needed)
    global.map.characters = data.characters; // add characters too

    // --- Optional debug ---
    show_debug_message("[handle_init_map] Map loaded: "
        + string(array_length(places)) + " places, "
        + string(array_length(routes)) + " routes.");

    if (global.DEBUG_ENABLED) {
        for (var i = 0; i < array_length(places); i++) {
            show_debug_message("- " + places[i].name + " at (" 
                + string(places[i].loc.x) + ", " + string(places[i].loc.y) + ")");
        }
    }
}