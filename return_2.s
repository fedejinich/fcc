	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0	sdk_version 12, 1
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
; %bb.0:
	mov	w0, #2
	ret
                                        ; -- End function
.subsections_via_symbols
