import {Computer} from "computer";

const computer = Computer.new()



const memoryList = document.getElementById("memory-list");

const fillMemoryList = () => {
    const memory = computer.memory_read();
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

const fillInstructionList = () => {
    const instructions = computer.instructions_read();
    console.log(instructions)
}

fillInstructionList()
fillMemoryList()
const test = computer.input(" x    = 2;");
console.log(test)