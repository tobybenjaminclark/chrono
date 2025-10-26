// Always keep anchors correct in case scale changed
_recalc_anchors();

// Single source of motion
x = lerp(x, target_x, 0.20);

// Snap if close (prevents asymptotic hang)
if (abs(x - target_x) < 0.5) x = target_x;

switch (state)
{
    case "hidden":
        // shrink while hidden/off
        image_xscale = lerp(image_xscale, 0.12, 0.2);
        image_yscale = lerp(image_yscale, 0.12, 0.2);

        // Fully invisible when off-screen
        if (x >= off_x - 0.5) {
            visible = false;
            appear_timer = 0;
            image_angle = 0;
        }

        // Become available
        if (global.has_new_ledgers) {
            state = "button";
            visible = true;

            // re-init entry
            image_xscale = 0.22;
            image_yscale = 0.22;
            _recalc_anchors();
            target_x = on_x;   // slide onto screen
            appear_timer = 0;
        }
        break;

    case "button":
        // keep resting target on-screen (don’t let anything overwrite it)
        target_x = on_x;

        // subtle wiggle
        appear_timer += wiggle_speed;
        image_angle = sin(appear_timer) * wiggle_amount;

        // click to expand
        if (mouse_check_button_pressed(mb_left)) {
            if (point_in_rectangle(
                mouse_x, mouse_y,
                x - spr_w * image_xscale * 0.5,
                y - spr_h * image_yscale * 0.5,
                x + spr_w * image_xscale * 0.5,
                y + spr_h * image_yscale * 0.5))
            {
                state = "expanded";
                expand_amount = 0;

                var accept = instance_create_layer(x + 80, y - 30, "UI", oButtonAccept);
                var deny   = instance_create_layer(x + 80, y + 30, "UI", oButtonReject);
                accept.ledger_parent = id;
                deny.ledger_parent   = id;
            }
        }
        break;

    case "expanded":
        // lock the envelope in place while the panel is open
        target_x = on_x;
        expand_amount = lerp(expand_amount, 1, 0.1);
        break;
}
