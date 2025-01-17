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
test: 
	PUSH RBP
	MOV RBP, RSP
	SUB RSP, 16
	MOV RAX, QWORD [RBP + 16]
	MOV QWORD [RBP - 16], RAX
	MOV RAX, QWORD [rbp-16]
	PUSH RAX
	CALL print
	MOV RAX, QWORD [rbp-16]
	PUSH RAX
	PUSH 1
	POP RBX
	POP RAX
	ADD RAX, RBX
	PUSH RAX
	CALL test
	MOV RSP, RBP
	POP RBP
	RET 
main: 
	PUSH RBP
	MOV RBP, RSP
	SUB RSP, 16
	PUSH 3
	CALL test
	XOR EAX, EAX
	LEAVE 
	RET 
