/// @function init_faction_timeline()
/// @desc Generates random faction control changes (0–1 timeline) for each location.
///       Example: global.faction_timeline["Nottingham Castle"] = [ {t:0.0,f:"g"}, {t:0.6,f:"t"} ];
function init_faction_timeline()
{
    if (!variable_global_exists("map") || !variable_struct_exists(global.map, "ownership")) {
        show_debug_message("[init_faction_timeline] No map ownership data!");
        exit;
    }

    global.faction_timeline = {};

    var locations = variable_struct_get_names(global.map.ownership);
    for (var i = 0; i < array_length(locations); i++)
    {
        var name = locations[i];
        var base_owner = global.map.ownership[$ name];
        var timeline = [ { t: 0.0, f: base_owner } ];

        // --- Random number of ownership changes (0–3) ---
        var n_changes = irandom_range(0, 3);

        var factions = [ "c", "g", "t" ]; // clerics, gnomes, traders for example
        var last_faction = base_owner;

        for (var j = 0; j < n_changes; j++)
        {
            var new_time = random_range(0.1, 0.95); // avoid exact 0/1
            var new_faction;

            // pick a different faction than before
            repeat (10) {
                new_faction = factions[irandom(array_length(factions) - 1)];
                if (new_faction != last_faction) break;
            }

            array_push(timeline, { t: new_time, f: new_faction });
            last_faction = new_faction;
        }

        // sort by time
        timeline = array_sort(timeline, function(a, b) { return a.t - b.t; });

        // final stability after 1.0 (optional)
        array_push(timeline, { t: 1.0, f: last_faction });

        global.faction_timeline[$ name] = timeline;
    }

    show_debug_message("[init_faction_timeline] Generated timelines for " + string(array_length(locations)) + " locations.");
}



/// @function get_faction(location, time)
/// @desc Returns the faction controlling a location at a given time.
function get_faction(location, time)
{
    var name = is_struct(location) ? location.name : string(location);
    var t = clamp(time, 0, 1);

    // --- Use generated timeline if available ---
    if (variable_global_exists("faction_timeline") &&
        variable_struct_exists(global.faction_timeline, name))
    {
        var timeline = global.faction_timeline[$ name];
        var current_faction = timeline[0].f;

        for (var i = 0; i < array_length(timeline); i++)
        {
            var entry = timeline[i];
            if (entry.t <= t) current_faction = entry.f;
            else break;
        }
        return current_faction;
    }

    // --- Fallback: static ownership ---
    if (variable_global_exists("map") && variable_struct_exists(global.map, "ownership")) {
        if (variable_struct_exists(global.map.ownership, name)) {
            return global.map.ownership[$ name];
        }
    }

    return "c"; // fallback
}
