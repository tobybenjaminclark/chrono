// Radius
radius = 5

// Instance variables
global.client_socket = network_create_socket(network_socket_tcp);
global.server_socket = network_connect_raw_async(global.client_socket, "127.0.0.1", 9999);

if (global.server_socket < 0) {
    show_message("Could not connect! Try turning on the server?");
} else {
    var t_buffer = buffer_create(128, buffer_grow, 1);
    buffer_seek(t_buffer, buffer_seek_start, 0);

    var hello_json = "{ \"HELLO\": {} }";
    buffer_write(t_buffer, buffer_string, hello_json);
    network_send_packet(global.client_socket, t_buffer, buffer_tell(t_buffer));
    buffer_delete(t_buffer);
}

audio_play_sound(backing, 0, true, 0.25);