import {Computer} from "computer";

const computer = Computer.new()

const memoryList = document.getElementById("memory-list");


const fillMemoryList = () => {
    const memory = computer.memory_read();
    memoryList.innerHTML = "";
    memory.forEach((value, key) => {
            const li = document.createElement("li");
            const div1 = document.createElement("div");
            const div2 = document.createElement("div");
            div1.innerText = key;
            div2.innerText = value;
            li.appendChild(div1);
            li.appendChild(div2);
            li.setAttribute("id", key);
            memoryList.appendChild(li);
    })
}

const instructionList = document.getElementById("instruction-list");

const fillInstructionList = () => {
    const instructions = computer.instructions_read();
    instructionList.innerHTML = "";
    instructions.forEach((value, key) => {
        const li = document.createElement("li");
        const div1 = document.createElement("div");
        const div2 = document.createElement("div");
        div1.innerText = key;
        div2.innerText = value;
        li.appendChild(div1);
        li.appendChild(div2);
        li.setAttribute("id", key);
        instructionList.appendChild(li);
    })
}

const inputBtn = document.getElementById("input-btn")
const handleInput = () => {
    const inputElement = document.getElementById("input");
    if (inputElement.value) {
        computer.input(inputElement.value);
    }
    inputElement.value = "";
    fillMemoryList();
    fillInstructionList();
}
inputBtn.addEventListener("click", handleInput)

const pcOut = document.getElementById("pc-out");
const cirOut = document.getElementById("cir-out");
const marOut = document.getElementById("mar-out");
const mdrOut = document.getElementById("mdr-out");

const updatePcOut = () => {
    pcOut.innerText = computer.counter_get().toString();
}

const updateMiddle = () => {
    cirOut.innerText = computer.cir_get();
    marOut.innerText = computer.mar_get().toString();
    mdrOut.innerText = computer.mdr_get();
}

const controlUnitOut = document.getElementById("control-unit-out");
const accumulatorOut = document.getElementById("accumulator-out");
const aluOut = document.getElementById("alu-out");

const updateLeft = () => {
    controlUnitOut.innerText = computer.cir_get();
    accumulatorOut.innerText = computer.accumulator_get().toString();
    aluOut.innerText = computer.alu_get();
}


const step = () => {
    computer.step();
    fillMemoryList();
    updatePcOut();
    updateMiddle();
    updateLeft();
}

updatePcOut()
const stepBtn = document.getElementById("step-btn");
stepBtn.addEventListener("click", step);
