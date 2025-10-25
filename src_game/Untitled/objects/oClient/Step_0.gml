/// Step Event — connection heartbeat
if (!global.is_connected) exit; // nothing to do if already offline

// send a small "PING" packet every second
if (!variable_global_exists("ping_timer")) global.ping_timer = 0;
global.ping_timer += delta_time / 1000000; // convert µs → seconds

if (global.ping_timer > 2) // every ~1 second
{
    global.ping_timer = 0;

    var buffer = buffer_create(32, buffer_grow, 1);
    buffer_write(buffer, buffer_string, "{ \"PING\": {} }");

    var send_ok = network_send_packet(global.client_socket, buffer, buffer_tell(buffer));
    buffer_delete(buffer);

    // If send fails → socket is dead
    if (send_ok <= 0) {
        show_debug_message("Lost connection — marking offline");
        global.is_connected = false;
    }
}
