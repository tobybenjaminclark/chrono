/// @function draw_active_events_list(_margin, _y)
/// @desc Draws the names and descriptions of all currently active events
///       at global.VIEWTIME, right-aligned to the screen edge,
///       with a "Current Events" header.
function draw_active_events_list(_margin, _y)
{
    if (!variable_global_exists("events") || array_length(global.events) == 0)
        exit;

    var t = clamp(global.VIEWTIME, 0, 1);
    var margin = _margin;
    var y_ = _y;
    var line_gap = 4;
    var name_h = 20;
    var desc_h = 32;
    var block_h = name_h + desc_h + line_gap;
    var box_w = 280;

    // --- Collect active events (excluding baseline) ---
    var active = [];
    for (var i = 0; i < array_length(global.events); i++)
    {
        var ev = global.events[i];
        if (ev.is_active(t) && ev.name != "Baseline")
            array_push(active, ev);
    }

    if (array_length(active) == 0) exit;

    // --- Positioning ---
    var screen_w = display_get_gui_width();
    var x_right = screen_w - margin;
    var x_left  = x_right - box_w;

    // --- Header ---
    draw_set_halign(fa_right);
    draw_set_valign(fa_top);

    // Use a bigger font if available
    draw_set_font(fnt_title);
    draw_set_color(c_yellow);
    draw_text(x_right, y_ - 36, "Current Events");

    // --- Box ---
    var box_h = array_length(active) * block_h + 8;
    draw_set_color(make_color_rgb(0, 0, 0));
    draw_set_alpha(0.5);
    draw_rectangle(x_left, y_ - 4, x_right + 4, y_ + box_h, false);
    draw_set_alpha(1);

    // --- Event text ---
    draw_set_font(fnt_troll);
    draw_set_halign(fa_right);
    draw_set_valign(fa_top);

    for (var j = 0; j < array_length(active); j++)
    {
        var ev = active[j];
        var y_line = y_ + j * block_h;

        var name_col = ev.is_active(t) ? c_yellow : c_white;
        draw_set_color(name_col);
        draw_text(x_right - 10, y_line, ev.name);

        draw_set_color(make_color_rgb(180, 180, 200));
        draw_text_ext(x_right - 20, y_line + name_h - 2, ev.description, 15, 800);
    }
}
