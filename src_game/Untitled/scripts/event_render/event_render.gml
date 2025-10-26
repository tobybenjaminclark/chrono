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


/// @function get_character_status(char_name, t)
/// @desc Returns "DEAD", "MAYBE", or "ALIVE" based on t vs any death events for the character.
function get_character_status(char_name, t)
{
    if (!variable_global_exists("events") || !is_array(global.events))
        return "ALIVE";

    var evs = global.events;
    var saw_past_death_end = false;

    for (var i = 0; i < array_length(evs); i++)
    {
        var ev = evs[i];
        if (!is_struct(ev) || !variable_struct_exists(ev, "effects")) continue;

        // Support either {start,end} or {start_time,end_time}
        var start_t =  undefined;
        var end_t   =  undefined;
        if (variable_struct_exists(ev, "start"))      start_t = ev.start;
        if (variable_struct_exists(ev, "end"))        end_t   = ev.end;
        if (is_undefined(start_t) && variable_struct_exists(ev, "start_time")) start_t = ev.start_time;
        if (is_undefined(end_t)   && variable_struct_exists(ev, "end_time"))   end_t   = ev.end_time;
        if (is_undefined(start_t) || is_undefined(end_t)) continue;

        var effects = ev.effects;
        if (!is_array(effects)) continue;

        for (var j = 0; j < array_length(effects); j++)
        {
            var eff = effects[j];
            if (is_struct(eff) && variable_struct_exists(eff, "Death") && eff.Death == char_name)
            {
                // At time t, is the death event ongoing?
                if (t >= start_t && t <= end_t) {
                    return "MAYBE";
                }
                // Otherwise remember if a death has already completed before t.
                if (end_t < t) {
                    saw_past_death_end = true;
                }
            }
        }
    }

    return saw_past_death_end ? "DEAD" : "ALIVE";
}





/// @function draw_events_circle(_x, _y, _r_scale)
/// @desc Draws all global.events as circular tracks around the map position.
///       Time runs 0–1 from 135° → 405° clockwise (over the top).
///       Highlights events intersecting global.VIEWTIME.
function draw_events_circle(_x, _y, _r)
{
    // --- Safety checks ---
    if (!variable_global_exists("events") || is_undefined(global.events)) exit;
    if (!variable_global_exists("map_radius") || is_undefined(global.map_radius)) exit;
    if (!variable_global_exists("VIEWTIME") || is_undefined(global.VIEWTIME)) exit;

    var cx = _x;
    var cy = _y;
    var r  = _r;

    var base_r = r + 10;
    var tview = clamp(global.VIEWTIME, 0, 1);

    var evs = global.events;
    var ev_count = array_length(evs);
    if (ev_count == 0) exit;

    // --- Draw each event arc ---
    for (var i = 0; i < ev_count; i++)
    {
        var ev = evs[i];
        if (!is_struct(ev) || !variable_struct_exists(ev, "start_time")) continue;

        var track_offset = (ev.track * 10);
        var rr = base_r + track_offset;

        var active = (tview >= ev.start_time) && (tview <= ev.end_time);
        var col = active ? c_yellow : c_white;
        var thickness = (active ? 4 : 2);

        // Convert to radians (135°–405° = top arc)
        var start_angle = degtorad(135 + (ev.start_time * 270));
        var end_angle   = degtorad(135 + (ev.end_time   * 270));

        // --- Draw the arc ---
        draw_event_arc(cx, cy, rr, start_angle, end_angle, thickness, col);

        // --- Draw start/end markers ---
        var x1 = cx + rr * cos(start_angle);
        var y1 = cy + rr * sin(start_angle);
        var x2 = cx + rr * cos(end_angle);
        var y2 = cy + rr * sin(end_angle);

        draw_set_color(active ? c_yellow : c_red);
        draw_circle(x1, y1, 2, false);
        draw_circle(x2, y2, 2, false);
    }
}
