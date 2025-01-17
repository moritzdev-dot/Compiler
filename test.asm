section .data
	fmt db "%d ", 0
section .text
global main
extern printf
print: 
	PUSH RBP
	MOV RBP, RSP
	MOV RDI, fmt
	MOV RSI, [RBP + 16]
	XOR RAX, RAX
	CALL printf
	LEAVE 
	RET 
main: 
	PUSH RBP
	MOV RBP, RSP
	SUB RSP, 16
	PUSH 3
	POP RAX
	MOV QWORD [rbp-8], RAX
	MOV RAX, QWORD [rbp-8]
	PUSH RAX
	PUSH 5
	POP RBX
	POP RAX
	CMP RAX, RBX
	SETG AL
	MOVZX RAX, AL
	PUSH RAX
	POP RAX
	CMP RAX, 0
	JE .A0
	MOV RAX, QWORD [rbp-8]
	PUSH RAX
	PUSH 3
	POP RBX
	POP RAX
	CMP RAX, RBX
	SETL AL
	MOVZX RAX, AL
	PUSH RAX
	POP RAX
	CMP RAX, 0
	JE .A1
	PUSH 1
	CALL print
	JMP .A2
.A1: 
	PUSH 2
	CALL print
.A2: 
	JMP .A3
.A0: 
	PUSH 3
	CALL print
.A3: 
	XOR EAX, EAX
	LEAVE 
	RET 
