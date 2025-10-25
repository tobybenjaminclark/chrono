function draw_route_curve(route, steps, color)
{	
    draw_set_alpha(1);
    draw_set_color(color);

    var count = array_length(route);
    if (count < 2) return;

    for (var i = 0; i < count - 1; i++) {
        var a = route[i];
        var b = route[i + 1];
        draw_line(a.x, a.y, b.x, b.y);
    }

}
