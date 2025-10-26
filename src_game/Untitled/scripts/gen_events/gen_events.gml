function gen_events(n) {
    if (global.client_socket != undefined) {
        var t_buffer = buffer_create(4096, buffer_grow, 1);
        buffer_seek(t_buffer, buffer_seek_start, 0);

        // Build a struct instead of concatenating strings
        var data = {
            GEN_EVENTS: {
                events: global.events,
                characters: global.characters,
                n: n
            }
        };

        // Convert the struct into a JSON string
        var json_string = json_stringify(data);

        // Write to buffer and send
        buffer_write(t_buffer, buffer_string, json_string);
        network_send_packet(global.client_socket, t_buffer, buffer_tell(t_buffer));
        buffer_delete(t_buffer);
    } else {
        show_message("No active TCP connection!");
    }
}




/// @function handle_init_map(data)
/// @desc Builds global.map from server-sent JSON structure
function handle_gen_events(data)
{
	global.ledgers = [
        Ledger(
			data._events[array_length(data._events) - 1].name,
			data._events[array_length(data._events) - 1].description,
			function(){},
			data._events,
	)];
    global.has_new_ledgers = true;
}