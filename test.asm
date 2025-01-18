section .data
.S0: 
	DB "HALLO", 0
.S1: 
	DB "%s", 0
.S2: 
	DB "%d", 0

section .text
global main
extern printf
print: 
	PUSH RBP
	MOV RBP, RSP
	MOV RDI, [RBP + 16]
	MOV RSI, [RBP + 24]
	XOR RAX, RAX
	CALL printf
	LEAVE 
	RET 
main: 
	PUSH RBP
	MOV RBP, RSP
	SUB RSP, 16
	MOV RAX, .S0
	PUSH RAX
	POP RAX
	MOV QWORD [rbp-8], RAX
	MOV RAX, QWORD [rbp-8]
	PUSH RAX
	MOV RAX, .S1
	PUSH RAX
	CALL print
	PUSH 3
	MOV RAX, .S2
	PUSH RAX
	CALL print
	XOR EAX, EAX
	LEAVE 
	RET 
