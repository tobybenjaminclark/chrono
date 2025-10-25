visible = false;

image_xscale = 0.01;
image_yscale = 0.01;

x_target = x;        // final resting position
x_start  = x + 200;  // start off-screen to the right
x = x_start;

appear_timer   = 0;   // for animation control
wiggle_speed   = 0.15;
wiggle_amount  = 4;   // rotation angle wiggle
wiggle_scale   = 0.1; // ±10% scaling wiggle
