// Reset state and clear the canvas
reboot();

// Set the default color to green
//
// When sampling out of bound pixels, if `wrap`
// isn't toggled on, it will use the color set
// by calling `defaultColor`, defaulting to black.
defaultColor([255, 0, 255]);

// Rotate the canvas
rotate(0.01 * TAU);

// Render to the canvas
render();

// Press `Shift + Enter` to execute
