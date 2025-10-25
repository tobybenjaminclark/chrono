/// @function get_faction(location, time)
/// @desc Returns the faction associated with a given location name or Place struct,
///       evolving over time (gnomes expand their territory).
function get_faction(location, time)
{
    var name = is_struct(location) ? location.name : string(location);
    var t = clamp(time, 0, 1); // ensure 0 ≤ t ≤ 1

    switch (name)
    {
        // Always gnomes
        case "Byrons Manor":
        case "Hills":
            return "g";

        // Turf originally controlled by traders ("t")
        // but gnomes take over after t > 0.6
        case "Riflery":
            return (t > 0.6) ? "g" : "t";

        // Coastal cities ("c") — gnomes take them near the end
        case "Farm":
            return (t > 0.5) ? "g" : "c";

        case "Market Square":
            return (t > 0.7) ? "g" : "c";

        case "Harbour":
            return (t > 0.9) ? "g" : "c";

        // Default — at the very end the gnomes have taken everything
        default:
            return (t > 0.95) ? "g" : "c";
    }
}
