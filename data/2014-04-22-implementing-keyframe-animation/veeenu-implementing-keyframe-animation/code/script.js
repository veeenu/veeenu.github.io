/*
  Copyright (C) 2014 Andrea Venuta <http://veeenu.github.io>
  All rights reserved.

  Redistribution and use in source and binary forms, with or without 
  modification, are permitted provided that the following conditions 
  are met:

    1. Redistributions of source code must retain the above copyright 
       notice, this list of conditions and the following disclaimer.
    2. Redistributions in binary form must reproduce the above copyright 
       notice, this list of conditions and the following disclaimer in the 
      documentation and/or other materials provided with the distribution.

  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS 
  IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
  THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR 
  PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR 
  CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, 
  EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, 
  PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR 
  PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF 
  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING 
  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS 
  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

*/

(function() {
  "use strict";

  // Initialize WebGL context
  var gl = (function() {

    var canvas = document.getElementById('canvas');
    canvas.style.position = 'fixed';

    var gl = canvas.getContext('webgl') || 
             canvas.getContext('experimental-webgl') || 
             console.log('WebGL unsupported');

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    gl.viewport(0, 0, canvas.width, canvas.height);

    window.addEventListener('resize', function() {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;

      gl.viewport(0, 0, canvas.width, canvas.height);

    });

    return gl;

  }());

  // Compile the shader program, pack the uniform locations in it and
  // return the whole thing
  var prog = (function(gl, attribs, uniforms) {

    var compileShader = function(source, type) {
      var xhr = new XMLHttpRequest();
      xhr.open('GET', source, false);
      xhr.send();

      var shader = gl.createShader(type);
      gl.shaderSource(shader, xhr.responseText);
      gl.compileShader(shader);
      console.log('Shader compilation:', gl.getShaderInfoLog(shader));

      return shader;
    }

    var program = gl.createProgram();
    gl.attachShader(program, 
                    compileShader('vertex.glsl', gl.VERTEX_SHADER));
    gl.attachShader(program, 
                    compileShader('fragment.glsl', gl.FRAGMENT_SHADER));

    gl.linkProgram(program);
    gl.useProgram(program);

    for(var i in attribs)
      program[attribs[i]] = gl.getAttribLocation(program, attribs[i]);

    for(var i in uniforms)
      program[uniforms[i]] = gl.getUniformLocation(program, uniforms[i]);

    return program;

  }(
    gl,
    ['aVertex', 'aNormal'],
    ['uM', 'uV', 'uP', 'uN']
  ));

  // Parse Wavefront OBJ file format.
  function parseObj(t) {
    var vertices = [], fverts = [], indices = [], 
        normals = [], fnormals = [], flist = [], 
        texc = [], ftexc = [];

    for(var i = 0; i < t.length; i++) {
      var line = t[i], m = null;

      if(m = line.match(/^v ([^\s]+)\s([^\s]+)\s([^\s]+)/)) {
        fverts.push(parseFloat(m[1]));
        fverts.push(parseFloat(m[2]));
        fverts.push(parseFloat(m[3]));
      } else if(m = line.match(/^vn ([^\s]+)\s([^\s]+)\s([^\s]+)/)) {
        fnormals.push(parseFloat(m[1]));
        fnormals.push(parseFloat(m[2]));
        fnormals.push(parseFloat(m[3]));
      } else if(m = line.match(/^vt ([^\s]+)\s+([^\s]+)/)) {
        ftexc.push(parseFloat(m[1]));
        ftexc.push(parseFloat(m[2]));
      } else if(m = line.match(/^f (\d+)\/(\d*)\/(\d+)\s+(\d+)\/(\d*)\/(\d+)\s+(\d+)\/(\d*)\/(\d+)/)) {
        var i1 = m[1], i2 = m[4], i3 = m[7], 
            n1 = m[3], n2 = m[6], n3 = m[9],
            f1 = m[2], f2 = m[5], f3 = m[8];

        i1--; i2--; i3--;
        n1--; n2--; n3--;
        f1--; f2--; f3--;
        flist.push([i1, n1, f1]);
        flist.push([i2, n2, f2]);
        flist.push([i3, n3, f3]);

      }

    }

    for(var i = 0; i < flist.length; i++) {
      var f = flist[i];
      indices.push(i);
      vertices.push(fverts[f[0] * 3]);
      vertices.push(fverts[f[0] * 3 + 1]);
      vertices.push(fverts[f[0] * 3 + 2]);
      normals.push(fnormals[f[1] * 3]);
      normals.push(fnormals[f[1] * 3 + 1]);
      normals.push(fnormals[f[1] * 3 + 2]);
      texc.push(ftexc[f[2] * 2]);
      texc.push(ftexc[f[2] * 2 + 1]);
    }

    return { 
      vertices: vertices, 
      normals: normals, 
      indices: indices, 
      textureCoordinates: texc // unused here
    };
  }

  var Mesh = (function(gl, program) {

    var Mesh = function(geometry) {
      this.geometry = geometry;

      this.vertexBuffer  = gl.createBuffer();
      this.indexBuffer   = gl.createBuffer();
      this.normalBuffer  = gl.createBuffer();

      gl.bindBuffer(gl.ARRAY_BUFFER, this.vertexBuffer);
      gl.bufferData(gl.ARRAY_BUFFER, 
                    new Float32Array(this.geometry.vertices), 
                    gl.STATIC_DRAW
                   );
      gl.bindBuffer(gl.ARRAY_BUFFER, this.normalBuffer);
      gl.bufferData(gl.ARRAY_BUFFER, 
                    new Float32Array(this.geometry.normals), 
                    gl.STATIC_DRAW
                   );
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);
      gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, 
                    new Uint16Array(this.geometry.indices), 
                    gl.STATIC_DRAW
                   );

      gl.bindBuffer(gl.ARRAY_BUFFER, null);
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
    }

    Mesh.prototype.draw = function(baseMatrix) {
      var mvMatrix = mat4.create();
      var nMatrix  = mat3.create();

      if(typeof baseMatrix !== 'undefined') {
        mvMatrix = mat4.clone(baseMatrix);
      } else {
        mat4.identity(mvMatrix);
        mat4.translate(mvMatrix, mvMatrix, this.geometry.position);
        mat4.rotate(mvMatrix, mvMatrix, 
                    this.geometry.rotate[0], [1.0, 0.0, 0.0]);
        mat4.rotate(mvMatrix, mvMatrix, 
                    this.geometry.rotate[1], [0.0, 1.0, 0.0]);
        mat4.rotate(mvMatrix, mvMatrix, 
                    this.geometry.rotate[2], [0.0, 0.0, 1.0]);
        mat4.scale(mvMatrix, mvMatrix, 
                   this.geometry.scale || [0.6, 0.6, 0.6]);
      }

      mat3.normalFromMat4(nMatrix, mvMatrix);

      gl.uniformMatrix4fv(program.uM, false, mvMatrix);
      gl.uniformMatrix3fv(program.uN, false, nMatrix);

      gl.bindBuffer(gl.ARRAY_BUFFER, this.vertexBuffer);
      gl.vertexAttribPointer(program.aVertex, 3, gl.FLOAT, false, 0, 0);
      gl.enableVertexAttribArray(program.aVertex);

      gl.bindBuffer(gl.ARRAY_BUFFER, this.normalBuffer);
      gl.vertexAttribPointer(program.aNormal, 3, gl.FLOAT, false, 0, 0);
      gl.enableVertexAttribArray(program.aNormal);

      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);

      gl.drawElements(this.geometry.rendering, 
                      this.geometry.indices.length, 
                      gl.UNSIGNED_SHORT, 0
                     );

      gl.bindBuffer(gl.ARRAY_BUFFER, null);
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
    }

    return Mesh;

  }(gl, prog));

  function lerpGeometry(a, b, t) {
  
    var out = {}, i;

    // Copy non strictly geometry-related properties in the output.
    // This is specific to the format used in this code and may or
    // may not be useful in other contexts.
    for(i in a) if(a.hasOwnProperty(i)) out[i] = a[i];

    // Assign new, empty arrays to our geometric properties
    out.vertices = [];
    out.indices = [];
    out.normals = [];

    // Loop through the indices "just like" the GL pipeline would
    for(i = 0; i < a.indices.length; i++) {
      // Current index. Same index should match the same vertices on
      // both a and b geometries
      var index = a.indices[i];

      // Push the index as-is
      out.indices.push(a.indices[i]);

      // Lerp the vertices. Take (1-t) % from a, (t) % from b.
      // Note that t should always be between 0 and 1
      out.vertices.push(
        // X component
        a.vertices[index * 3] * (1 - t) + 
        b.vertices[index * 3] * t,

        // Y component
        a.vertices[index * 3 + 1] * (1 - t) + 
        b.vertices[index * 3 + 1] * t,

        // Z component
        a.vertices[index * 3 + 2] * (1 - t) + 
        b.vertices[index * 3 + 2] * t
      );

      // Lerp the normal. Same as the vertex
      out.normals.push(
        a.normals[index * 3] * (1 - t) + 
        b.normals[index * 3] * t,

        a.normals[index * 3 + 1] * (1 - t) + 
        b.normals[index * 3 + 1] * t,

        a.normals[index * 3 + 2] * (1 - t) + 
        b.normals[index * 3 + 2] * t
      );
    }

    return out;

  }

  // objFile holds the four keyframes we're going to use
  var meshes = [], objFiles = [
    "skeletal_000000.obj",
    "skeletal_000010.obj",
    "skeletal_000020.obj",
    "skeletal_000030.obj"
  ];

  for(var i = 0; i < objFiles.length; i++)
    (function(obj, i) {
    
      // XHRs are sync here because we must push the meshes in the correct
      // order. It's okay here because we don't need anything fancy but
      // you should never do this in the real world and opt for a correct
      // asynchronous behaviour, but this is not the place to discuss it.
      var xhr = new XMLHttpRequest();
      xhr.open('GET', 'frames/' + obj, false);
      xhr.onreadystatechange = function() {
        if(this.readyState !== 4) return;

        var t = this.response.split('\n');
        var geom = parseObj(t); // parse file
        geom.rendering = gl.TRIANGLES;
        
        if(meshes.length > 0) {
          // If this is not the first object, then the pos/rot/scale should
          // all reference the pos/rot/scale on the very first mesh, so
          // we can alter these and the changes will propagate through the
          // meshes. There are way better ways to do this, but still
          geom.position = meshes[0].geometry.position;
          geom.rotate = meshes[0].geometry.rotate;
          geom.scale = meshes[0].geometry.scale;

          // Pick the last geometry from the previous iteration
          var prevGeom = meshes[meshes.length - 1].geometry;
          // Get ten geometries linearly interpolated (lerped) between
          // the current and the previous keyframes
          for(var k = 1; k < 10; k++) {
            var lg = lerpGeometry(prevGeom, geom, k/10);
            meshes.push(new Mesh(lg));
          }

        } else {
          geom.position = [0.0, 0.0, 0.0];
          geom.rotate = [0.0, 0.0, 0.0];
          geom.scale = [1.0, 1.0, 1.0];
          meshes.push(new Mesh(geom));
        }

        // Finally, lerp last and first keyframe to get a
        // seamless animation
        if(i == objFiles.length - 1) for(var k = 0; k < 10; k++) {
          var lg = lerpGeometry(geom, meshes[0].geometry, k/10);
          meshes.push(new Mesh(lg));
        }
      } 
      xhr.send();

    }(objFiles[i], i));

  var uP = mat4.create(), uV = mat4.create();
  mat4.perspective(uP, 45, 
                   window.innerWidth / window.innerHeight, 
                   0.01, 500.0
                  );
  mat4.identity(uV);
  mat4.translate(uV, uV, [0, -2, -10.0]);

  gl.clearColor(0.0, 0.0, 0.0, 1.0);
  gl.enable(gl.DEPTH_TEST);

  var curMesh = 0, curMeshf = 0.0;

  (function animate(time) {

    var delta = time - (animate.timeOld || time);

    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

    gl.uniformMatrix4fv(prog.uP, false, uP);
    gl.uniformMatrix4fv(prog.uV, false, uV);

    if(meshes.length > 0 && meshes[curMesh] !== undefined) {
      meshes[0].geometry.rotate[1] += 0.02;
      meshes[curMesh].draw();
    }

    if(meshes.length > 0) {
      if(!isNaN(delta)) curMeshf += 0.05 * delta;
      curMesh = parseInt(curMeshf);
      curMesh %= meshes.length;
    }

    requestAnimationFrame(animate);
    animate.timeOld = time;

  }(0));
  
}());
