+++
template = "article.html"
title = "A little fun with LeapMotion"
aliases = ["/2014/04/10/testing-leapmotion.html"]
+++

I recently bought a [LeapMotion](https://www.leapmotion.com/). It's a great little tool, although interacting with it has somewhat of a learning curve. The API on the other hand has quite a beautiful design, is very well documented and a couple of minutes of reading are enough to get started. <!-- more -->

I'll have to take an exam about neural networks this summer, so later on
I'll begin working on a neural network based gesture recognition system and
will discuss it here.

Below you can see a very small example; if you own a LeapMotion, plug
it in, point your finger towards the screen and move it around to draw. 
Despite `leapd` being quite heavy, and thus having a few hiccups on my somewhat
old machine, the data flows seamlessly and the curves drawn are quite smooth.
Depending on the task, developing an algorithm to compensate for unreasonably
large jumps in position may be necessary -- the script in this page
just stitches together the points as long as you stay past the hover area,
maybe discarding a point if the distance between that and the one before that
is larger than a set threshold would do the trick. Also I'm not really sure if
the data collected by the LeapMotion comes in already normalized or using
something like an inverse perspective projection matrix might do some good, I
perceive some dissonance between my movements and the results on the screen
when I use it but I can't really say if it's just a bit of lag. Will definitely
have to look into it.

Anyway here's the canvas:

<canvas id='canvas-2014-04-10-testing-leapmotion' width='400' height='300'></canvas>
<script src="//js.leapmotion.com/leap-0.4.3.js"></script>
<script type='text/javascript' src='/data/2014-04-10/testing-leapmotion.js'></script>

And you get to read the source code, too. ;)

```js
(function() {

  "use strict";

  var main = function() {

    var ctl = new Leap.Controller({ enableGestures: false }),
        output = document.getElementById('canvas-2014-04-10-testing-leapmotion'),
        outCtx = output.getContext('2d'),
        paths = [[]],
        tipPoint = { x: 0, y: 0 };

    output.width = 400;
    output.height = 300;
    output.style.display = 'block';
    output.style.position = 'relative';
    output.style.margin = 'auto';
    output.style.background = 'rgba(255, 255, 255, 0.3)';

    ctl.on('frame', function(evt) {
      if(evt.fingers.length === 0) return;

      var pos = null, maxl = 0;
      for(var i = 0; i < evt.fingers.length; i++)
        if(evt.fingers[i].length > maxl) {
          maxl = evt.fingers[i].length;
          pos = evt.fingers[i].tipPosition;
        }

      var lastPath = paths[paths.length - 1];

      if(pos[2] < 0) {
        lastPath.push({ x: pos[0] + 200, y: 400 - pos[1] });
      } else if(lastPath.length > 0 && lastPath[lastPath.length - 1] !== null){
        paths.push([]);
      }
      tipPoint.x = pos[0] + 200;
      tipPoint.y = 400 - pos[1];

    });

    ctl.connect();

    output.addEventListener('mousedown', function(evt) {
      // Cleanup
      while(paths.length)
        paths.shift();
      paths.push([]);
      outCtx.clearRect(0, 0, 400, 300);
    });

    requestAnimationFrame(function anim() {
      outCtx.save();
      outCtx.clearRect(0, 0, 400, 300);
      for(var i = 0; i < paths.length; i++) {

        outCtx.beginPath();
        for(var j = 0; j < paths[i].length; j++)
          if(j === 0)
            outCtx.moveTo(paths[i][j].x, paths[i][j].y);
          else
            outCtx.lineTo(paths[i][j].x, paths[i][j].y);
        outCtx.stroke();
      }
      outCtx.restore();
      requestAnimationFrame(anim);
    });
  };


  if(document.readyState === 'complete')
    main();
  else
    window.onload = main;

}());
```