var t = ds_map_find_value(async_load, "type");

switch (t)
{
    case network_type_connect:
        global.is_connected = true;
        break;

    case network_type_disconnect:
        global.is_connected = false;
        break;

    case network_type_data:
		global.is_connected = true;
        var t_buffer = ds_map_find_value(async_load, "buffer");
        var cmd_type = buffer_read(t_buffer, buffer_string);
        var jsonData = json_parse(string(cmd_type));

        if (variable_struct_exists(jsonData, "INIT_MAP"))
        {
            handleInitMap(jsonData.INIT_MAP);
        }

        show_debug_message(jsonData);
        break;
}
