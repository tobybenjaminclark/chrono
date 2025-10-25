/// @function draw_road(x1, y1, x2, y2)
/// @desc Draws a thick road segment with an outline.
function draw_road(x1, y1, x2, y2)
{
    var width       = 3;         // inner road width
    var outline_col = make_color_rgb(255, 255, 255);
    var road_col    = make_color_rgb(150, 150, 150);
    var outline_w   = width + 4;  // total width including outline

    var dx  = x2 - x1;
    var dy  = y2 - y1;
    var len = point_distance(x1, y1, x2, y2);
    if (len <= 0) return;

    var nx = -dy / len;
    var ny =  dx / len;

    // --- Draw outline layer first ---
    var hw = outline_w * 0.5;
    var x1a = x1 + nx * hw;
    var y1a = y1 + ny * hw;
    var x1b = x1 - nx * hw;
    var y1b = y1 - ny * hw;
    var x2a = x2 + nx * hw;
    var y2a = y2 + ny * hw;
    var x2b = x2 - nx * hw;
    var y2b = y2 - ny * hw;

    draw_set_color(outline_col);
    draw_primitive_begin(pr_trianglestrip);
    draw_vertex(x1a, y1a);
    draw_vertex(x1b, y1b);
    draw_vertex(x2a, y2a);
    draw_vertex(x2b, y2b);
    draw_primitive_end();

    // --- Draw inner road on top ---
    hw = width * 0.5;
    x1a = x1 + nx * hw;
    y1a = y1 + ny * hw;
    x1b = x1 - nx * hw;
    y1b = y1 - ny * hw;
    x2a = x2 + nx * hw;
    y2a = y2 + ny * hw;
    x2b = x2 - nx * hw;
    y2b = y2 - ny * hw;

    draw_set_color(road_col);
    draw_primitive_begin(pr_trianglestrip);
    draw_vertex(x1a, y1a);
    draw_vertex(x1b, y1b);
    draw_vertex(x2a, y2a);
    draw_vertex(x2b, y2b);
    draw_primitive_end();
}
