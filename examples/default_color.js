// Reset state and clear the canvas
reboot();

// Set the default color to purple
//
// When sampling out of bound pixels, if `wrap`
// isn't set, it will use the color set by calling
// `defaultColor`, defaulting to black.
defaultColor([255, 0, 255]);

// Rotate the canvas
//
// This will sample out of bound pixels, and since `wrap`
// isn't set, it will use the default color that was set above.
rotate(0.01 * TAU);

// Render to the canvas
render();

// Press `Shift + Enter` to execute
