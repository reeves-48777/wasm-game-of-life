function loadShader(gl, type, source) {
    const shader = gl.createShader(type);

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        alert(`An error occured compiling the shaders : ${gl.getShaderInfoLog(shader)}`);
        gl.deleteShader(shader);
        return null;
    }

    return shader;
}

function hexToGlColor(hex) {
    if (hex.startsWith('#')) {
        hex = hex.substring(1);
    }

    const r = parseInt(hex.slice(0,2), 16) / 255;
    const g = parseInt(hex.slice(2,4), 16) / 255;
    const b = parseInt(hex.slice(4,6), 16) / 255;
    let a = 1.0;

    if (hex.length === 8) {
        a = parseInt(hex.slice(6,8), 16) / 255;
    }

    return [r, g, b, a];
}

export {loadShader, hexToGlColor};