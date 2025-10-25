/// @function Event(track, start_time, end_time, [name], [description])
/// @desc Creates a new Event struct with track, start/end times, and metadata.
///       All times are normalized to [0,1].
function Event(track, start_time, end_time, name, description)
{
    return {
        track: track,
        start_time: clamp(start_time, 0, 1),
        end_time: clamp(end_time, 0, 1),
        name: is_undefined(name) ? "Unnamed Event" : string(name),
        description: is_undefined(description) ? "" : string(description),

        /// @desc Check if this event is active at a given time (0–1)
        is_active: function(time) {
            time = clamp(time, 0, 1);
            return (time >= self.start_time) && (time <= self.end_time);
        },

        /// @desc Duration of the event (end - start)
        duration: function() {
            return self.end_time - self.start_time;
        }
    };
}


/// @function init_default_events()
/// @desc Creates some fake, time-bounded events (0–1) and assigns tracks.
///       Prepends a baseline event lasting from 0 → 1.
function init_default_events()
{
    global.events = [];

    // --- Helper to add events with auto track assignment ---
    var add_event = function(start_time, end_time, name, description)
    {
        var track = 0;
        var overlap = true;

        while (overlap)
        {
            overlap = false;
            for (var i = 0; i < array_length(global.events); i++)
            {
                var ev = global.events[i];
                if (ev.track == track)
                {
                    if ((start_time < ev.end_time) && (end_time > ev.start_time))
                    {
                        overlap = true;
                        break;
                    }
                }
            }
            if (overlap) track++;
        }

        array_push(global.events, Event(track, start_time, end_time, name, description));
    };

    // --- Prepend a baseline event covering 0 → 1 ---
    array_push(global.events, Event(0, 0, 1, "Baseline", "Covers the entire timeline."));

    // --- Create some fake events (randomised) ---
    randomize();
    for (var i = 0; i < 10; i++)
    {
        var s = random_range(0, 0.9);
        var e = min(s + random_range(0.05, 0.2), 1);

        var name = "Event " + string(i + 1);
        var desc = "Random event #" + string(i + 1) + 
                   " from " + string_format(s, 0, 2) + " → " + string_format(e, 0, 2);

        add_event(s, e, name, desc);
    }

    show_debug_message(
        "[init_default_events] Generated " + string(array_length(global.events)) + " fake events!"
    );
}

