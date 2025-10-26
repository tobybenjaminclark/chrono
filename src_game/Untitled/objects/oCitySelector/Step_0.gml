/// @desc City Carousel – STEP

// Compute dynamic label box (based on current city text size)
var txt = cities[city_index];
var tw = string_width(txt);
var th = string_height(txt);
var label_w = tw + pad_x * 2;
var label_h = th + pad_y * 2;

// Layout (centered on center_x/center_y)
var label_x = center_x - label_w * 0.5;
var label_y = center_y - label_h * 0.5;

var left_x1  = label_x - gap - arrow_w; // left arrow rect
var left_y1  = center_y - arrow_h * 0.5;
var left_x2  = left_x1 + arrow_w;
var left_y2  = left_y1 + arrow_h;

var right_x1 = label_x + label_w + gap; // right arrow rect
var right_y1 = left_y1;
var right_x2 = right_x1 + arrow_w;
var right_y2 = left_y2;

// Hover detection
hover_left  = point_in_rectangle(mouse_x, mouse_y, left_x1,  left_y1,  left_x2,  left_y2);
hover_right = point_in_rectangle(mouse_x, mouse_y, right_x1, right_y1, right_x2, right_y2);

// Mouse clicks
if (mouse_check_button_pressed(mb_left)) {
    if (hover_left) {
        city_index = (city_index - 1 + array_length(cities)) mod array_length(cities);
        selected_city = cities[city_index];
    } else if (hover_right) {
        city_index = (city_index + 1) mod array_length(cities);
        selected_city = cities[city_index];
    }
}

// Keyboard ← →
if (keyboard_check_pressed(vk_left)) {
    city_index = (city_index - 1 + array_length(cities)) mod array_length(cities);
    selected_city = cities[city_index];
}
if (keyboard_check_pressed(vk_right)) {
    city_index = (city_index + 1) mod array_length(cities);
    selected_city = cities[city_index];
}
