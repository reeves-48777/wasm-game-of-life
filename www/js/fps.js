const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // convert delta time since the last frame render into a measure of fps
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // save only the latest 100 timings
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // find the max, min, average of the timings
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let avg = sum / this.frames.length;

        this.fps.textContent = `
        Frames per Second:
    latest = ${Math.round(fps)}
    avg = ${Math.round(avg)}
    min = ${Math.round(min)}
    max = ${Math.round(max)}
        `.trim();
    }
}

export {fps};