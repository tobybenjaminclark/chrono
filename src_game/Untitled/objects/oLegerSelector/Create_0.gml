cards = [];
var spacing = 180;
var start_x = room_width / 2 - spacing;

for (var i = 0; i < array_length(global.ledgers); i++) {
    var l = global.ledgers[i];
    var card = instance_create_layer(start_x + i * spacing, room_height + 150, "Instances", oLedgerCard);
    card.ledger_data = l;
    card.target_y = room_height / 2;
    array_push(cards, card);
}

transition = 0;
