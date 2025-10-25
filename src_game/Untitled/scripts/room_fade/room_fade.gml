/// @function fade_to_room(target)
function fade_to_room(target)
{
    if (!instance_exists(oTransition)) instance_create_layer(0,0,"GUI",oTransition);
    var tr = instance_find(oTransition, 0);
    tr.transition_state = "out";
    tr.next_room = target;
}