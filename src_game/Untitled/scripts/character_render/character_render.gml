/// @function lighten_color(col, amt)
/// @desc Mixes color with white (amt ∈ [0,1])
function lighten_color(col, amt)
{
    var r = color_get_red(col);
    var g = color_get_green(col);
    var b = color_get_blue(col);
    r = lerp(r, 255, amt);
    g = lerp(g, 255, amt);
    b = lerp(b, 255, amt);
    return make_color_rgb(r, g, b);
}




/// @function draw_characters_circle(_x, _y, _r, _scale)
/// @desc Draws all global.characters as sprites along the top arc above the map.
///       Names are displayed on the outside, angled with the circle.
///       `_scale` adjusts positions fully but sprite/text sizes partially.
function draw_characters_circle(_x, _y, _r, _scale)
{
    if (!variable_global_exists("characters") || array_length(global.characters) == 0)
        exit;

    var cx = _x;
    var cy = _y;
    var r  = _r;
    var sc = _scale;

    // --- Determine outer radius (beyond all events) ---
    var outer_r = r + 10 * sc;
    if (variable_global_exists("events") && array_length(global.events) > 0)
    {
        var max_track = 0;
        for (var i = 0; i < array_length(global.events); i++)
        {
            var ev = global.events[i];
            if (variable_struct_exists(ev, "track"))
                max_track = max(max_track, ev.track);
        }
        outer_r += (max_track + 1) * 10 * sc;
    }


    // --- Copy + sort characters by race ---
    var chars = array_create(array_length(global.characters));
    array_copy(chars, 0, global.characters, 0, array_length(global.characters));

    array_sort(chars, function(a, b) {
        var order = ["t", "g", "c"]; // Troll, Gnome, Centaur
        var ai = -1;
        var bi = -1;
        for (var j = 0; j < array_length(order); j++) {
            if (order[j] == a.faction) ai = j;
            if (order[j] == b.faction) bi = j;
        }
        if (ai == -1) ai = 999;
        if (bi == -1) bi = 999;
        return ai - bi;
    });

    var count = array_length(chars);
    if (count == 0) exit;

    // --- Arc geometry ---
    var arc_mid = degtorad(270);
    var base_spread = 60;
    var spread_per_char = 10;
    var total_spread = base_spread + max(0, count - 3) * spread_per_char;
    var spread = degtorad(total_spread);
    var step = (count > 1) ? (spread / (count - 1)) : 0;
    var start_angle = arc_mid - spread * 0.5;

    // --- Precompute positions ---
    var positions = array_create(count);
    for (var i = 0; i < count; i++)
    {
        var ang = start_angle + i * step;
        var rr = outer_r + 30 * sc;
        var px = cx + rr * cos(ang);
        var py = cy + rr * sin(ang);
        positions[i] = [px, py];
    }

    // --- Draw connecting lines ---
    draw_set_alpha(0.7);
    for (var i = 1; i < count; i++)
    {
        var c_prev = chars[i - 1];
        var c_curr = chars[i];

        if (c_prev.faction != c_curr.faction)
            continue;

        var col;
        switch (c_curr.faction)
        {
            case "g": col = make_color_rgb(0x56, 0x6D, 0x27); break;
            case "t": col = make_color_rgb(0x72, 0x64, 0x7b); break;
            case "c": col = make_color_rgb(0xb2, 0x88, 0x60); break;
            default:  col = c_white; break;
        }

        draw_set_color(col);
        var p1 = positions[i - 1];
        var p2 = positions[i];
        draw_line_width(p1[0], p1[1], p2[0], p2[1], 10 * sc);
    }
    draw_set_alpha(1);

    // --- Draw each character ---
    for (var i = 0; i < count; i++)
    {
        var c = chars[i];
        var spr = get_faction_sprite(c.faction);

        var ang = start_angle + i * step;
        var rr = outer_r + 30 * sc;
        var px = positions[i][0];
        var py = positions[i][1];

        // --- Faction colour ---
        var col = c_white;
        switch (c.faction) {
            case "g": col = make_color_rgb(0x56, 0x6D, 0x27); break;
            case "t": col = make_color_rgb(0x72, 0x64, 0x7b); break;
            case "c": col = make_color_rgb(0xb2, 0x88, 0x60); break;
        }
        draw_set_color(col);

        // --- Partial scaling ---
        var base_sprite_scale = 0.20;
        var spr_scale = base_sprite_scale * lerp(1, sc, 0.5); // 50% responsive to zoom
        var text_scale = lerp(1, sc, 0.25);                   // 25% responsive to zoom

        // --- Sprite background circle ---
        draw_circle(
            px,
            py,
            (sprite_get_width(spr) * spr_scale / 2) + 2 * sc,
            false
        );

        // --- Sprite ---
        draw_sprite_ext(
            spr, 0,
            px, py,
            spr_scale, spr_scale,
            0, c_white, 1
        );
		
		var status = get_character_status(c.name, global.VIEWTIME);
		if status == "DEAD" {
		    draw_sprite_ext(
	            sprSkull, 0,
	            px, py,
	            spr_scale * 0.9, spr_scale* 0.9,
	            0, c_white, 0.85
	        );	
		}
		if status == "MAYBE" {
		    draw_sprite_ext(
	            sprBlood, 0,
	            px, py,
	            spr_scale * 0.9, spr_scale* 0.9,
	            0, c_white, 0.65
	        );	
		}

        // --- Text placement ---
        draw_set_halign(fa_center);
        draw_set_valign(fa_middle);

        var text_offset = sprite_get_height(spr) * (spr_scale);
        var text_r = rr + text_offset;
        var text_x = cx + text_r * cos(ang);
        var text_y = cy + text_r * sin(ang);

        var rot = radtodeg(-(ang - pi / 2));
        rot = (rot % 360 + 360) % 360;
        if (rot > 90 && rot < 270) rot += 180;

        var light_col = lighten_color(col, 0.6);
		
		draw_set_color(light_col);
        
        draw_text_transformed(text_x, text_y, c.name, text_scale, text_scale, rot);
    }

    // --- Reset draw state ---
    draw_set_halign(fa_left);
    draw_set_valign(fa_top);
    draw_set_color(c_white);
}

