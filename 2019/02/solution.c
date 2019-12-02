#include <stdio.h>
#include <stdlib.h>

int initial_program[] = {1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,2,9,19,23,1,23,6,27,1,13,27,31,1,31,10,35,1,9,35,39,1,39,9,43,2,6,43,47,1,47,5,51,2,10,51,55,1,6,55,59,2,13,59,63,2,13,63,67,1,6,67,71,1,71,5,75,2,75,6,79,1,5,79,83,1,83,6,87,2,10,87,91,1,9,91,95,1,6,95,99,1,99,6,103,2,103,9,107,2,107,10,111,1,5,111,115,1,115,6,119,2,6,119,123,1,10,123,127,1,127,5,131,1,131,2,135,1,135,5,0,99,2,0,14,0};
int program_size = sizeof(initial_program) / sizeof(int);

int *program_copy() {
    int *result = malloc(program_size);
    int i;
    for (i = 0; i < program_size; i++) {
        result[i] = initial_program[i];
    }

    return result;
}

void restore(int *program, int noun, int verb) {
    program[1] = noun;
    program[2] = verb;
}

void run(int * program) {
    // program counter
    int pc;
    for (pc = 0; pc < program_size; pc += 4) {
        int op1, op2;
        switch (program[pc])
        {
        case 1:
            op1 = program[program[pc+1]];
            op2 = program[program[pc+2]];
            program[program[pc+3]] = op1 + op2;
            break;
        case 2:
            op1 = program[program[pc+1]];
            op2 = program[program[pc+2]];
            program[program[pc+3]] = op1 * op2;
            break;
        case 99:
            // stop execution
            return;
        default:
            printf("invalid opcode: %d\n", program[pc]);
            exit(1);
        }
    }
}

int main() {
    int *program = program_copy();
    restore(program, 12, 2);
    run(program);
    printf("After execution, program[0] is %d\n", program[0]);
    free(program);
}
