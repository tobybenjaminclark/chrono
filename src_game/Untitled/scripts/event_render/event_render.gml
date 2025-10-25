/// @function draw_event_arc(cx, cy, r, start_angle, end_angle, thickness, color)
/// @desc Draws an arc between two angles with a given thickness.
function draw_event_arc(cx, cy, r, start_angle, end_angle, thickness, color)
{
    var segs = 36;
    var step = (end_angle - start_angle) / segs;

    draw_set_color(color);

    // Draw multiple arcs outward to simulate thickness
    for (var t = 0; t < thickness; t++)
    {
        var rr = r + t - thickness * 0.5; // centre thickness around r
        var prevx = cx + rr * cos(start_angle);
        var prevy = cy + rr * sin(start_angle);

        for (var s = 1; s <= segs; s++)
        {
            var a = start_angle + step * s;
            var _x = cx + rr * cos(a);
            var _y = cy + rr * sin(a);
            draw_line(prevx, prevy, _x, _y);
            prevx = _x;
            prevy = _y;
        }
    }
}


/// @function draw_events_circle()
/// @desc Draws all global.events as circular tracks around the map.
///       Time runs 0–1 from 135° → 405° clockwise (over the top).
///       Highlights events intersecting global.VIEWTIME.

function draw_events_circle()
{
    if (!variable_global_exists("events")) exit;

    var cx = room_width  div 2;
    var cy = room_height div 2;
    var base_r = global.map_radius + 10;

    var tview = clamp(global.VIEWTIME, 0, 1);

    for (var i = 0; i < array_length(global.events); i++)
    {
        var ev = global.events[i];
        var r  = base_r + (ev.track * 10);

        var active = (tview >= ev.start_time) && (tview <= ev.end_time);
        var col = active ? c_yellow : c_white;
        var thickness = active ? 4 : 2;

        // Convert to radians
        var start_angle = degtorad(135 + (ev.start_time * 270));
        var end_angle   = degtorad(135 + (ev.end_time   * 270));

        // ✅ draw the arc
        draw_event_arc(cx, cy, r, start_angle, end_angle, thickness, col);

        // Draw start/end markers
        var x1 = cx + r * cos(start_angle);
        var y1 = cy + r * sin(start_angle);
        var x2 = cx + r * cos(end_angle);
        var y2 = cy + r * sin(end_angle);

        draw_set_color(active ? c_yellow : c_red);
        draw_circle(x1, y1, 2, false);
        draw_circle(x2, y2, 2, false);
    }
}
