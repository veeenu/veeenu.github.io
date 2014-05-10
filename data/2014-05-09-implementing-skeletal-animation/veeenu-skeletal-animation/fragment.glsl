precision highp float;

uniform mat4 uP, uV, uM;
uniform mat4 uBones[32];
uniform mat3 uN;
uniform bool uSkin;

varying vec3 vVertex, vNormal;

void main() {

  vec3 lAmbient = vec3(0.3, 0.3, 1.0);
  vec3 lDiffuse = vec3(0.3, 0.3, 1.0);
  vec3 lSpecular= vec3(1.0, 1.0, 1.0);

  vec3 plPos = vec3(0.0, 16.0, 48.0);
  vec3 plDir = normalize(plPos - vVertex);

  mat4 mvp = uP * uV * uM;
  vec3 n = normalize(uN * vNormal);
  vec3 l = normalize(vec3(vec4(plDir, 1.0)));
  vec3 v = normalize(-vec3(vec4(vVertex, 1.0)));
  vec3 r = reflect(l, n);

  float lambert = dot(l, n),
        ambientInt = 0.1,
        specularInt = 0.0,
        diffuseInt = 0.9,
        shininess = 1024.0;

  float specular = pow( max( 0.0, dot(r,v) ), shininess );

  gl_FragColor = vec4(
      lAmbient * ambientInt +
      lDiffuse * diffuseInt * lambert +
      lSpecular * specularInt * specular
      , 1.0);
}
