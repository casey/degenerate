# Programming

Degenerate programs are written in JavaScript and sent to a Web Worker for
execution. The program then sends back a series of `State` objects from the
worker, which are used to configure image filters that the renderer applies in
the main thread.

The JavaScript API is concerned with setting properties of the current `State`
object, sending `State` objects to the main thread, and populating the sidebar
with interactive widgets.

## Image Filters

iterative rendering

#### Transformation

#### Mask

#### Operation

#### Alpha

#### Wrap

#### Default Color

## API

```javascript
{{#include ../../www/worker.js}}
```
