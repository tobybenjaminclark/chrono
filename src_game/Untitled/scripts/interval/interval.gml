/// @function get_faction(location, time)
/// @desc Returns the faction controlling a location at a given time.
///       At t=0, uses the initial ownership provided by the server.
///       Later times can evolve control dynamically (e.g. gnome conquest).
function get_faction(location, time)
{
    var name = is_struct(location) ? location.name : string(location);
    var t = clamp(time, 0, 1); // ensure 0 ≤ t ≤ 1

    // --- Base ownership from the map data ---
    var base_owner = "c"; // fallback
    if (variable_global_exists("map") && variable_struct_exists(global.map, "ownership")) {
        if (variable_struct_exists(global.map.ownership, name)) {
            base_owner = global.map.ownership[$ name];
        }
    }

    // --- Evolution rules over time ---
    switch (base_owner)
    {
        case "g":
            // Gnomes always stay gnomes
            return "g";

        case "t":
            // Traders lose turf to gnomes after 0.6
            return (t > 0.6) ? "g" : "t";

        case "c":
            // Coastal cities slowly fall to gnomes after 0.8
            return (t > 0.8) ? "g" : "c";

        default:
            // Unknown → eventually taken by gnomes
            return (t > 0.95) ? "g" : base_owner;
    }
}
