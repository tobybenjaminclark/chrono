/// @function Character(_name, _faction)
/// @desc Basic character struct for map entities
function Character(_name, _faction)
{
    return {
        name    : _name,
        faction : _faction
    };
}


/// @function init_default_characters()
/// @desc Initializes a default set of 3 characters (Troll, Gnome, Centaur)

function init_default_characters()
{
    // Make sure Character() exists
    if (!is_undefined(Character))
    {
        global.characters = [
            Character("Gruk the Troll", "t"),
            Character("Fizzlepuff the Gnome", "g"),
            Character("Orynth the Centaur", "c")
        ];

        show_debug_message("[init_default_characters] Spawned 3 default characters!");
    }
    else
    {
        show_debug_message("[init_default_characters] ERROR: Character struct not defined!");
        global.characters = [];
    }
}
