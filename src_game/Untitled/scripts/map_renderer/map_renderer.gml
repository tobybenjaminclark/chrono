/// @function draw_map(_map, _x, _y, _r)
/// @desc Draws routes (with optional quadratic curves) and circular locations.
///       All coordinates are assumed to be in [-1, 1].
function draw_map(_map, _x, _y, _r)
{
    if (is_undefined(_map)) return;

    var cx = _x;
    var cy = _y;
    var r  = _r;

    // --- Draw boundary circle (unit radius) ---
    draw_set_color(make_color_rgb(60, 60, 60));
    draw_set_circle_precision(32);
    draw_circle(cx, cy, r, false);

    // --- Draw routes (roads) ---
    var routes = _map.routes;
    draw_set_alpha(1);
    draw_set_color(make_color_rgb(120, 120, 120));

	var routes = _map.routes;
	for (var i = 0; i < array_length(routes); i++) {
	    var route = routes[i];
	    var count = array_length(route);
	    if (count < 2) continue;

	    // duplicate endpoints for edge cases
	    var x0 = route[0].x, y0 = route[0].y;
	    var x1, y1, x2, y2, x3, y3;

	    for (var j = 0; j < count - 1; j++) {
	        x1 = route[max(0, j - 1)].x;
	        y1 = route[max(0, j - 1)].y;
	        x2 = route[j].x;
	        y2 = route[j].y;
	        x3 = route[min(count - 1, j + 1)].x;
	        y3 = route[min(count - 1, j + 1)].y;

	        // convert to screen space once
	        var p0x = cx + x1 * r;
	        var p0y = cy - y1 * r;
	        var p1x = cx + x2 * r;
	        var p1y = cy - y2 * r;
	        var p2x = cx + x3 * r;
	        var p2y = cy - y3 * r;

	        var steps = 20;
	        var prevx = p1x;
	        var prevy = p1y;

	        for (var t_i = 1; t_i <= steps; t_i++) {
	            var t = t_i / steps;
	            // Catmull–Rom to Bezier approximation
	            var t2 = t * t;
	            var t3 = t2 * t;

	            var qx = 0.5 * ((2 * p1x) +
	                            (-p0x + p2x) * t +
	                            (2*p0x - 5*p1x + 4*p2x - (cx + route[min(count - 1, j + 2)].x * r)) * t2 +
	                            (-p0x + 3*p1x - 3*p2x + (cx + route[min(count - 1, j + 2)].x * r)) * t3);

	            var qy = 0.5 * ((2 * p1y) +
	                            (-p0y + p2y) * t +
	                            (2*p0y - 5*p1y + 4*p2y - (cy - route[min(count - 1, j + 2)].y * r)) * t2 +
	                            (-p0y + 3*p1y - 3*p2y + (cy - route[min(count - 1, j + 2)].y * r)) * t3);

	            draw_road(prevx, prevy, qx, qy);
	            prevx = qx;
	            prevy = qy;
	        }
	    }
	}

	// --- Draw locations (buildings) ---
	var locs = _map.locations;
	for (var k = 0; k < array_length(locs); k++) {
	    var p  = locs[k];
	    var px = cx + p.loc.x * r;
	    var py = cy - p.loc.y * r;

	    // --- Determine faction + sprite ---
	    var faction = get_faction(p, global.VIEWITME);
	    var code    = string_lower(string_char_at(faction, 1));
	    var spr     = get_faction_sprite(code);

	    // --- Draw scaled sprite ---
	    var spr_w = sprite_get_width(spr);
	    var spr_h = sprite_get_height(spr);
	    var scale = 0.25;

	    draw_sprite_ext(
	        spr, 0,
	        px, py,
	        scale, scale,
	        0,
	        c_white, 1
	    );

	    // --- Text setup ---
	    draw_set_font(get_faction_font(faction));
	    draw_set_halign(fa_center);
	    draw_set_valign(fa_bottom);

	    var text = p.name;
	    var text_y = py - spr_h * scale * 0.6;

	    // --- Measure text for background ---
	    var text_w = string_width(text);
	    var text_h = string_height(text);
	    var padding = 4; // padding around text

	    var rect_x1 = px - text_w / 2 - padding;
	    var rect_y1 = text_y - text_h - padding;
	    var rect_x2 = px + text_w / 2 + padding;
	    var rect_y2 = text_y + padding;

	    // --- Draw background rectangle ---
	    draw_set_color(make_color_rgb(0, 0, 0)); // black background
	    draw_rectangle(rect_x1, rect_y1, rect_x2, rect_y2, false);

	    // --- Draw text on top ---
	    draw_set_color(c_white);
	    draw_text(px, text_y, text);
	}

	// Reset alignment (always good practice)
	draw_set_halign(fa_left);
	draw_set_valign(fa_top);
}
