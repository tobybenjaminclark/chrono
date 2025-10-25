/// @function draw_thick_line(x1, y1, x2, y2, width, color)
/// @desc Draws a thick filled line segment using a quad.
function draw_road(x1, y1, x2, y2, color)
{
	var width = 10;
    var dx = x2 - x1;
    var dy = y2 - y1;
    var len = point_distance(x1, y1, x2, y2);
    if (len <= 0) return;

    var nx = -dy / len; // perpendicular normalized vector
    var ny =  dx / len;

    var hw = width * 0.5;

    var x1a = x1 + nx * hw;
    var y1a = y1 + ny * hw;
    var x1b = x1 - nx * hw;
    var y1b = y1 - ny * hw;
    var x2a = x2 + nx * hw;
    var y2a = y2 + ny * hw;
    var x2b = x2 - nx * hw;
    var y2b = y2 - ny * hw;

    draw_primitive_begin(pr_trianglestrip);
    draw_set_color(color);
    draw_vertex(x1a, y1a);
    draw_vertex(x1b, y1b);
    draw_vertex(x2a, y2a);
    draw_vertex(x2b, y2b);
    draw_primitive_end();
}
