/// @function get_faction(location)
/// @desc Returns the faction associated with a given location name or Place struct.
function get_faction(location, time)
{
    var name = is_struct(location) ? location.name : string(location);

    switch (name)
    {
        case "Byrons Manor":
        case "Hills":
            return "g";

        case "Riflery":
            return "t";

        case "Farm":
        case "Market Square":
        case "Harbour":
            return "c";

        default:
            return "g";
    }
}
