import { POSITION, RESOLUTION, SIZE } from "./consts";

const vsSource = `
attribute vec2 ${POSITION};
uniform vec2 ${RESOLUTION};
uniform float ${SIZE};

void main() {
    // convertir la position en clipspace
    vec2 clipspace = ((${POSITION} / ${RESOLUTION}) * 2.0) - 1.0;

    gl_Position = vec4(clipspace * vec2(1, -1), 0, 1);
    gl_PointSize = ${SIZE};
}
`;

export {vsSource};