/// @function get_faction_sprite(code)
/// @desc Maps a single-character faction code to its sprite.
/// @param {string} code - "g" = Gnome, "t" = Troll, "c" = Centaur
function get_faction_sprite(code)
{
    switch (string_lower(code))
    {
        case "g": return spr_gnome;
        case "t": return spr_troll;
        case "c": return spr_centaur;
        default:  return spr_gnome;
    }
}
