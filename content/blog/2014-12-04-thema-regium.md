+++
template = "article.html"
title = "Thema Regium, math rock and procedurally generated sound"
aliases = ["/2014/12/04/thema-regium.html"]
+++

No big secret, I'm a huge fan of procedural content generation. I'm also a big
fan of Johann Sebastian Bach and his _Musikalisches Opfer_, whose fascinating
story I came to know thanks to D.R. Hofstader's
["Gödel, Escher, Bach: an Eternal Golden Braid"](http://en.wikipedia.org/wiki/G%C3%B6del,_Escher,_Bach). <!-- more -->
On the side of it, the readers of this blog might also know I'm a huge fan of
Javascript, so at this point I don't see why putting the three things together
should be a bad idea.

These days I've been thinking of writing an engine that generates music from a
known set of rules, the same, a bit mechanical maybe, way I did while studying
musical composition: begin with a bass line melody starting from the fundamental
tone, move around and about in some way, one cadence after the other, finish
with the V-I cadence, then backtrack and harmonize the bass line (to err on the
side of simple: in the form of a choral). I know many put their compressed scores 
inside their demos and generate the sound according to the score, but I still 
haven't found pure randomly generated music (I didn't look very hard, admittedly).

Now for a philosophical quandary. You (I) could argue that human experience and
feelings are of fundamental importance in musical composition. From this
assertion alone could follow that letting a machine generate music may only
result in dull, soulless, mechanical sounds.

On the other hand, I firmly believe math to be a means of expressing
nature in a way transcending the human experience (and, oftentimes, knowledge) 
itself, and there's no shortage of examples - fractals, for instance, are
"nothing but" beautiful mathematical representations of beautiful natural
phenomena. So I'm going to try it and we'll see what comes out.

Anyway I began prototyping a few things, got to work and, after a few minutes of
tinkering, the following came out:

```javascript
var ac  = new window.AudioContext(),
    osc = ac.createOscillator(),
    notes = {
      'c': 261.6,
      'c#': 277.2,
      'db': 277.2,
      'd': 293.7,
      'd#': 311.1,
      'eb': 311.1,
      'e': 329.6,
      'f': 349.2,
      'f#': 370.0,
      'gb': 370.0,
      'g': 392.0,
      'g#': 415.3,
      'ab': 415.3,
      'a': 440.0,
      'a#': 466.2,
      'bb': 466.2,
      'b': 493.9,
      '--': 1
    },
    toPrint = function(note) {
      var out = note.charAt(0).toUpperCase();
      if(out === '-') return '';
      if(note.length > 1) {
        if(note.charAt(1) === 'b')
          out += '&#9837;';
        else
          out += '&#9839;';
      }
      return out;
    },
    themaRegium;

// "Thema Regium" beat by Bach feat. Frederich II of Prussia
// note octave duration note octave duration ...
themaRegium = 
  'c  1   2  eb 1  2   g  1  2   ab 1 2    b  0.5 2  -- 1 1  g  1 1 ' +
  'f# 1   2  f  1  2   e  1  2   eb 1 3    d  1   1  db 1 1  c  1 1 ' +
  'b  .5  1  a  .5 .5  g  .5 .5  c  1 1    f  1   1  eb 1 2  d  1 2 ' +
  'c 1 4';
themaRegium = themaRegium.split(/\s+/g);


var nextNote = function() {
  if(themaRegium.length < 1) {
    document.body.innerHTML += '!'; 
    return osc.stop(); 
  }

  var note = themaRegium.shift(),
      oct  = parseFloat(themaRegium.shift()),
      dur  = parseFloat(themaRegium.shift()) * 250;
      console.log(note, dur);

  osc.frequency.value = notes[note] * oct;
  setTimeout(nextNote, dur);
  document.body.innerHTML += ' ' + toPrint(note);
}

osc.connect(ac.destination);
osc.type = 'triangle';
osc.start(0);
nextNote();
```

Basically it's a crude regular language in which all {%tex()%}i \% 3{%end%}th tokens represent
a note, all {%tex()%}i \% 3 + 1{%end%}th tokens represent the octave (expressed in a power of
2: e.g. lower octave is {%tex()%}\frac{1}{2}{%end%}, upper octave is {%tex()%}2{%end%}, next one is {%tex()%}4{%end%}
und so weiter) and all {%tex()%}i \% 3 + 2{%end%} tokens represent the duration. The
`nextNote` function is a finite state automaton that picks three values,
translates it to a frequency, plugs it in the oscillator, then waits for `dur *
250` ms and moves on.

You can read the full code [here](https://gist.github.com/veeenu/af48a4834ebc33306412)
and listen to it in action [here](http://bl.ocks.org/veeenu/af48a4834ebc33306412).

I'd love to hear from you; what's your stance in the "man vs machine in music"
discourse?
