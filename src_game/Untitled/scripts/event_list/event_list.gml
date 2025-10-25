/// @function draw_active_events_list(_x, _y)
/// @desc Draws the names and descriptions of all currently active events
///       at global.VIEWTIME. Typically shown on the left side of the screen.
function draw_active_events_list(_x, _y)
{
    if (!variable_global_exists("events") || array_length(global.events) == 0)
        exit;

    var t = clamp(global.VIEWTIME, 0, 1);
    var x_ = _x;
    var y_ = _y;
    var line_gap = 4;
    var name_h = 20;      // height for event name
    var desc_h = 16;      // height for description text
    var block_h = name_h + desc_h + line_gap;

    // --- Collect active events (excluding baseline) ---
    var active = [];
    for (var i = 0; i < array_length(global.events); i++)
    {
        var ev = global.events[i];
        if (ev.is_active(t) && ev.name != "Baseline")
            array_push(active, ev);
    }

    // --- Draw semi-transparent background ---
    if (array_length(active) > 0)
    {
        var box_w = 280;
        var box_h = array_length(active) * block_h + 8;
        draw_set_color(make_color_rgb(0, 0, 0));
        draw_set_alpha(0.5);
        draw_rectangle(x_ - 4, y_ - 4, x_ + box_w, y_ + box_h, false);
        draw_set_alpha(1);
    }

    // --- Draw text for each active event ---
    draw_set_halign(fa_left);
    draw_set_valign(fa_top);

    for (var j = 0; j < array_length(active); j++)
    {
        var ev = active[j];
        var y_line = y_ + j * block_h;

        // Highlight current event name
        var name_col = ev.is_active(t) ? c_yellow : c_white;
        draw_set_color(name_col);
        draw_text(x_, y_line, ev.name);

        // Description text (smaller and lighter)
        draw_set_color(make_color_rgb(180, 180, 200));
        draw_text(x_ + 10, y_line + name_h - 2, ev.description);
    }
}
