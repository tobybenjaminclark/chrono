switch (transition_state)
{
    case "in": // fade in
        transition_alpha = max(0, transition_alpha - transition_speed);
        break;

    case "out": // fade out
        transition_alpha = min(1, transition_alpha + transition_speed);
        if (transition_alpha >= 1 && next_room != -1) {
            // go to next room
            room_goto(next_room);
            transition_state = "in";    // prepare for fade-in in the new room
            next_room = -1;
        }
        break;
}
