


/// @function clamp_unit(val)
/// @desc Clamps a number to the range [-1, 1]
function clamp_unit(val) {
    return clamp(val, -1, 1);
}



/// @function Vec2(_x, _y)
/// @desc 2D vector normalised to [-1, 1]
function Vec2(_x, _y) constructor {
    x = clamp_unit(_x);
    y = clamp_unit(_y);
}



/// @function Place(_name, _x, _y)
/// @desc Place with a name and normalised coordinate
function Place(_name, _x, _y) constructor {
    name = _name;
    loc  = new Vec2(_x, _y);
}



/// @function Map(_places, _routes)
/// @desc Collection of places and routes (each Vec2 validated)
function Map(_places, _routes) constructor {
    locations = _places;
    
    routes = array_create(array_length(_routes));
    for (var i = 0; i < array_length(_routes); i++) {
        var route = _routes[i];
        var new_route = array_create(array_length(route));
        for (var j = 0; j < array_length(route); j++) {
            var p = route[j];
            if (is_struct(p)) new_route[j] = new Vec2(p.x, p.y);
            else new_route[j] = new Vec2(p[0], p[1]);
        }
        routes[i] = new_route;
    }
}



/// @function init_default_map()
/// @desc Creates a default global.map instance with richer, curving routes.
function init_default_map()
{
    // --- Clear any existing map ---
    if (variable_global_exists("map")) {
        global.map = undefined;
    }

    // --- Define locations (normalised coords in [-1, 1]) ---
    var forest  = new Place("Byrons Manor",  -0.65,  0.45);
    var lake    = new Place("Market Square",  0.75, -0.35);
    var village = new Place("Farm",            0.00,  0.05);
    var ruins   = new Place("Riflery",       -0.35, -0.75);
    var harbour = new Place("Harbour",         0.55,  0.55);
    var hills   = new Place("Hills",          -0.75, -0.05);

    var locations = [forest, lake, village, ruins, harbour, hills];

    // --- Define routes (curvy and branched) ---
    var route1 = [
        new Vec2(forest.loc.x,  forest.loc.y),
        new Vec2(-0.45,  0.55),   // curve upward
        new Vec2(-0.20,  0.35),
        new Vec2(village.loc.x,  village.loc.y)
    ];

    var route2 = [
        new Vec2(village.loc.x,  village.loc.y),
        new Vec2(0.35,   0.15),   // gentle bend toward Market
        new Vec2(0.55,  -0.10),
        new Vec2(lake.loc.x,     lake.loc.y)
    ];

    var route3 = [
        new Vec2(village.loc.x,  village.loc.y),
        new Vec2(-0.10, -0.25),
        new Vec2(-0.20, -0.55),
        new Vec2(ruins.loc.x,    ruins.loc.y)
    ];

    var route4 = [
        new Vec2(lake.loc.x,     lake.loc.y),
        new Vec2(0.60,   0.00),
        new Vec2(0.70,   0.35),
        new Vec2(harbour.loc.x,  harbour.loc.y)
    ];

    var route5 = [
        new Vec2(forest.loc.x,  forest.loc.y),
        new Vec2(-0.70,  0.20),
        new Vec2(-0.75,  0.00),
        new Vec2(hills.loc.x,  hills.loc.y)
    ];

    var route6 = [
        new Vec2(hills.loc.x,  hills.loc.y),
        new Vec2(-0.55, -0.25),
        new Vec2(-0.45, -0.50),
        new Vec2(ruins.loc.x, ruins.loc.y)
    ];

    var routes = [route1, route2, route3, route4, route5, route6];

    // --- Construct and assign map ---
    global.map = new Map(locations, routes);

    // --- Optional debug info ---
    if (global.DEBUG_ENABLED) {
        show_debug_message("[init_default_map] Global map initialised with "
            + string(array_length(locations)) + " locations and "
            + string(array_length(routes)) + " routes.");
    }

    return global.map;
}





/// @function draw_map_debug(map, x, y, size)
/// @desc Draws only the location names of the map (for debug).
function draw_map_debug(_map, _x, _y, _size)
{
    if (!global.DEBUG_ENABLED) return;
    if (is_undefined(_map)) return;

    var ox = _x;
    var oy = _y;
    var s  = _size; // map square size in pixels

    // Background
    draw_set_alpha(0.4);
    draw_set_color(c_black);
    draw_rectangle(ox - 4, oy - 4, ox + s + 4, oy + s + 4, false);


    // Coordinate transform: [-1,1] → [0,s]
    var tx = function(v, ox, s) { return ox + (v + 1) * 0.5 * s; };
    var ty = function(v, oy, s) { return oy + (1 - (v + 1) * 0.5) * s; };

    // --- Draw only location names ---
    draw_set_color(c_white);
    var locs = _map.locations;
    for (var k = 0; k < array_length(locs); k++) {
        var p = locs[k];
        var px = tx(p.loc.x, ox, s);
        var py = ty(p.loc.y, oy, s);
        draw_text(px + 6, py - 6, p.name);
    }

    // Header text
    draw_set_color(c_white);
    draw_text(ox, oy - 20, "[DEBUG MAP]");
}
