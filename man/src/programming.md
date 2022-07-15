# Programming

Degenerate programs are written in JavaScript and sent to a Web Worker for
execution. The program then sends back a series of `Filter` objects from the
worker thread, which are used to configure image filters that the renderer
applies in the main thread.

The JavaScript API is concerned with setting properties of the current `Filter`
object, sending `Filter` objects to the main thread, and populating the sidebar
with interactive widgets.

Individual image filters are relatively simple, but iterated application of one
or more filters can produce surprising and beautiful results, and varying image
filters over time or applying the same image filter in a loop can yield
striking animations.

## Image Filter Properties

Image filters read from a source image and write to a destination image. Every
time an image filter is applied, those images are swapped.

Image filters have a number of properties, including a coordinate
transformation, which determines whence input image pixels will be sampled; a
signed distance field, which determines which of those pixels will be modified;
and an color transformation, which determines how those pixels will be
modified.

For each pixel in the image, an image filter operates with roughly the
following steps:

1. Generate the coordinates of the current pixel
2. Transform those coordinates by the current transform
3. If wrapping is enabled and the transformed pixel coordinates are out of
   bounds, wrap them back in bounds
4. Sample the source image at those coordinates if they are in bounds,
   otherwise use the current default color
5. If the pixel is inside of the current SDf, apply the color transformation,
   otherwise use the original color
6. Save the generated pixel to the destination image

## API

```javascript
{{#include ../../www/worker.js}}
```
