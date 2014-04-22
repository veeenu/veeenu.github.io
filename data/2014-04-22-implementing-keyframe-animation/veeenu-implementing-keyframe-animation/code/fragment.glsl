precision highp float;

uniform mat4 uP, uV, uM;
uniform mat3 uN;

uniform sampler2D uSampler;
varying vec3 vVertex, vNormal;

void main() {

  vec3 lAmbient = vec3(1.0, 1.0, 1.0);
  vec3 lDiffuse = vec3(0.5, 0.5, 1.0);
  vec3 lSpecular= vec3(1.0, 1.0, 1.0);

  vec3 plPos = vec3(0.0, 3.0, 5.0);
  vec3 plDir = normalize(plPos - vVertex);

  mat4 mvp = uP * uV * uM;
  vec3 n = normalize(uN * vNormal);
  vec3 l = normalize(vec3(mvp * vec4(plDir, 1.0)));
  vec3 v = normalize(-vec3(mvp * vec4(vVertex, 1.0)));
  vec3 r = reflect(l, n);

  float lambert = dot(l, n),
        ambientInt = 0.2,
        specularInt = 0.5,
        diffuseInt = 1.0,
        shininess = 128.0;

  float specular = pow( max( 0.0, dot(r,v) ), shininess );

  gl_FragColor = vec4(
      lAmbient * ambientInt +
      lDiffuse * diffuseInt * lambert +
      lSpecular * specularInt * specular
      , 1.0);
}
