hover = point_in_rectangle(mouse_x, mouse_y, x - 70, y - 100, x + 70, y + 100);

if (hover && mouse_check_button_pressed(mb_left)) {
    // Apply ledger effect
    ledger_data.apply();
    global.has_new_ledgers = false;
    room_goto_previous();
}
