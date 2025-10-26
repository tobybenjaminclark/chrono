/// --- Ledger Button (Envelope) Setup ---

// Visibility & anim
visible       = false;
appear_timer  = 0;
wiggle_speed  = 0.15;
wiggle_amount = 4;
wiggle_scale  = 0.1;

// Start small (but not microscopic so you can see it animate)
image_xscale = 0.20;
image_yscale = 0.20;

// State
state         = "hidden";   // "hidden" | "button" | "expanded"
expand_amount = 0;

// Cache sprite metrics (DON'T trust sprite_width if sprite can change)
spr    = sprite_index;
spr_w  = sprite_get_width(spr);
spr_h  = sprite_get_height(spr);
spr_ox = sprite_get_xoffset(spr);
spr_oy = sprite_get_yoffset(spr);

// Motion
margin   = 25;     // pixels inside right edge when visible
target_x = x;      // unified motion target (never touch x in Draw)
on_x     = 0;      // recalculated each step
off_x    = 0;

// Recompute anchors given current scale/origin
function _recalc_anchors() {
    var sc = image_xscale;
    // distance from origin to RIGHT edge in pixels on screen
    var to_right = (spr_w - spr_ox) * sc;
    // distance from origin to LEFT edge
    var to_left  = (spr_ox) * sc;

    // visible resting x so RIGHT edge sits `margin` inside screen
    on_x  = room_width - to_right - margin;

    // fully off-screen to the right (left edge beyond room right)
    off_x = room_width + to_left + 8; // +8 safety to avoid 1px peeking
}

_recalc_anchors();
x        = off_x;     // start hidden off-screen
target_x = off_x;

// Handlers
function accept() {
    var _l = global.ledgers[0];
    if (_l != undefined) _l.apply();
    global.ledgers = [];

    global.has_new_ledgers = false;
    state = "hidden";
    expand_amount = 0;

    _recalc_anchors();
    target_x = off_x; // slide away
    // buttons clean-up
    with (oButtonAccept) if (ledger_parent == other.id) instance_destroy();
    with (oButtonReject) if (ledger_parent == other.id) instance_destroy();
}

function reject() {
    global.has_new_ledgers = false;
    state = "hidden";
    expand_amount = 0;
    global.ledgers = [];

    _recalc_anchors();
    target_x = off_x; // slide away
    with (oButtonAccept) if (ledger_parent == other.id) instance_destroy();
    with (oButtonReject) if (ledger_parent == other.id) instance_destroy();
}
