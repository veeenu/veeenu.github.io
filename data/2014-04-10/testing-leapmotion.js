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
