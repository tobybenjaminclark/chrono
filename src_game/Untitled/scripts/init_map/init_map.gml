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


function handle_init_map(data) {
    show_message("Handle init map");
}