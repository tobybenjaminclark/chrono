/// @function draw_characters_circle(_x, _y, _r)
/// @desc Draws all global.characters as sprites along the top arc above the map.
///       Names are displayed on the outside, angled with the circle.
function draw_characters_circle(_x, _y, _r)
{
    if (!variable_global_exists("characters") || array_length(global.characters) == 0)
        exit;

    var cx = _x;
    var cy = _y;
    var r  = _r;

    // --- Determine outer radius (beyond all events) ---
    var outer_r = r + 10;
    if (variable_global_exists("events") && array_length(global.events) > 0)
    {
        var max_track = 0;
        for (var i = 0; i < array_length(global.events); i++)
        {
            var ev = global.events[i];
            if (variable_struct_exists(ev, "track"))
                max_track = max(max_track, ev.track);
        }
        outer_r += (max_track + 1) * 10;
    }

    // --- Character placement parameters ---
    var chars = global.characters;
    var count = array_length(chars);
    if (count == 0) exit;

    var arc_mid = degtorad(270); // vertical top center

    // --- Dynamic spread (expands with count) ---
    var base_spread = 60;
    var spread_per_char = 10;
    var total_spread = base_spread + max(0, count - 3) * spread_per_char;
    var spread = degtorad(total_spread);

    var step = (count > 1) ? (spread / (count - 1)) : 0;
    var start_angle = arc_mid - spread * 0.5;

    // --- Draw each character ---
    for (var i = 0; i < count; i++)
    {
        var c = chars[i];
        var spr = get_faction_sprite(c.faction);

        // Compute position along the arc
        var ang = start_angle + i * step;
        var rr = outer_r + 30;

        var px = cx + rr * cos(ang);
        var py = cy + rr * sin(ang);

        // Draw sprite (centered)
        var scale = 0.20;
        draw_sprite_ext(
            spr, 0,
            px, py,
            scale, scale,
            0, c_white, 1
        );

		// --- Text setup ---
		draw_set_halign(fa_center);
		draw_set_valign(fa_middle);

		var col = c_white;
		switch (c.faction) {
		    case "t": col = c_red;  break;
		    case "g": col = c_lime; break;
		    case "c": col = c_aqua; break;
		}
		draw_set_color(col);

		// --- Text position and rotation (outside, tangent, readable) ---
		var text_offset = sprite_get_height(spr) * (scale  * 0.75);
		var text_r = rr + text_offset;
		var text_x = cx + text_r * cos(ang);
		var text_y = cy + text_r * sin(ang);

		// Tangent direction in GM screen coords (Y down): use ang - 90°
		var rot = radtodeg(-(ang - pi/2));


		// Keep labels upright: flip if pointing > 90° away from vertical
		// (normalize to [0,360) first for a robust check)
		rot = (rot % 360 + 360) % 360;
		if (rot > 90 && rot < 270) rot += 180;
		
		// Draw angled name
		draw_text_transformed(text_x, text_y, c.name, 1, 1, rot);
    }

    // Reset draw state
    draw_set_halign(fa_left);
    draw_set_valign(fa_top);
    draw_set_color(c_white);
}
