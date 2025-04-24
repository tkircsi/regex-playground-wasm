import init, { match_regex } from './wasm/regex_engine.js';

async function run() {
    await init(); // load WASM module

    const patternInput = document.getElementById('regex');
    const testInput = document.getElementById('test');
    const output = document.getElementById('output');

    function formatMatch(match) {
        return `"${match.value}" (position ${match.start}-${match.end})`;
    }

    function update() {
        try {
            const matches = match_regex(patternInput.value, testInput.value);
            if (matches.length === 0) {
                output.innerText = "No matches found";
            } else {
                output.innerText = matches.map(formatMatch).join('\n');
            }
        } catch (err) {
            output.innerText = "Error: " + err;
        }
    }

    patternInput.addEventListener('input', update);
    testInput.addEventListener('input', update);
}

run();
