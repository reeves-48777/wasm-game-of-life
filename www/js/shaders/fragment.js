import { COLOR } from "./consts";


const fsSource = `
precision mediump float;

uniform vec4 ${COLOR};

void main() {
    gl_FragColor = ${COLOR};
}
`;

export {fsSource};