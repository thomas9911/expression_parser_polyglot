import * as wasm from "expression_parser_web_assembly";

const printToElement = (elem, value) => {
    elem.innerHTML = value + " => " + JSON.stringify(wasm.parse_and_eval(value, null, 2));
};

const pre = document.createElement("pre");
pre.id = "text_output";

const input = document.createElement("textarea");
input.size = 50;
input.placeholder = "1 + 2";

const input_text = "[1 + 2, sin(12)]";

input.defaultValue = input_text;

input.oninput = () => {
    const pre = document.getElementById("text_output");
    const value = input.value;

    if (value) {
        printToElement(pre, value);
    }
}

printToElement(pre, input.value);

document.body.appendChild(input);
document.body.appendChild(pre);