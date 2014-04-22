uniform mat4 uP, uV, uM;
uniform mat3 uN;

attribute vec3 aVertex, aNormal;
attribute vec2 aTexCoord;

varying vec3 vVertex, vNormal; 

void main() {

  gl_Position = uP * uV * uM * vec4(aVertex, 1.0);
  vVertex = aVertex;
  vNormal = aNormal;

}
