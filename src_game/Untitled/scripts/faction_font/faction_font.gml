/// @function get_faction_sprite(code)
/// @desc Maps a single-character faction code to its sprite.
/// @param {string} code - "g" = Gnome, "t" = Troll, "c" = Centaur
function get_faction_font(code)
{
    switch (string_lower(code))
    {
        case "g": return fnt_troll;
        case "t": return fnt_troll;
        case "c": return fnt_troll;
        default:  return spr_gnome;
    }
}
