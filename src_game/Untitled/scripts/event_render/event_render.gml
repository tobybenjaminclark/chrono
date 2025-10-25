/// @function draw_events_circle()
/// @desc Draws all global.events as circular tracks around the map.
///       Time runs 0–1 from 135° → 405° clockwise (over the top).

function draw_events_circle()
{
    if (!variable_global_exists("events")) exit;

    var cx = room_width  div 2;
    var cy = room_height div 2;
    var base_r = global.map_radius + 10;

    draw_set_color(c_white);

    for (var i = 0; i < array_length(global.events); i++)
    {
        var ev = global.events[i];
        var r  = base_r + (ev.track * 10);

        // Convert start/end to radians (0–1 → 135°→405°)
        var start_angle = degtorad(135 + (ev.start_time * 270));
        var end_angle   = degtorad(135 + (ev.end_time   * 270));

        // Draw arc as short line segments
        var segments = 36;
        var step = (end_angle - start_angle) / segments;

        var prevx = cx + r * cos(start_angle);
        var prevy = cy + r * sin(start_angle);

        for (var s = 1; s <= segments; s++)
        {
            var a = start_angle + step * s;
            var _x = cx + r * cos(a);
            var _y = cy + r * sin(a);

            draw_line(prevx, prevy, _x, _y);
            prevx = _x;
            prevy = _y;
        }

        // Draw start/end markers
        var x1 = cx + r * cos(start_angle);
        var y1 = cy + r * sin(start_angle);
        var x2 = cx + r * cos(end_angle);
        var y2 = cy + r * sin(end_angle);

        draw_set_color(c_red);
        draw_circle(x1, y1, 2, false);

        draw_set_color(c_lime);
        draw_circle(x2, y2, 2, false);
    }
}
