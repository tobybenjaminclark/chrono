// Slide cards upward from bottom
transition = min(1, transition + 0.05);

for (var i = 0; i < array_length(cards); i++) {
    var c = cards[i];
    c.y = lerp(room_height + 150, c.target_y, transition);
}
