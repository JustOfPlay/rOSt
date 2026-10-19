.section .multiboot2
.align 8

header_start:
    .long 0xe85250d6
    .long 0
    .long header_end - header_start
    .long -(0xe85250d6 + (header_end - header_start))

    .short 0
    .short 0
    .long 8

header_end:

.section .text
.code32
.global _start
.extern kernel_main

_start:
    cli

    # Page tables prep
    mov eax, offset pdpt_table
    or eax, 3
    mov dword ptr [pml4_table], eax

    mov eax, offset pd_table
    or eax, 3
    mov dword ptr [pdpt_table], eax

    mov edi, offset pd_table
    mov eax, 0x83
    mov ecx, 512

map_pages:
    mov dword ptr [edi], eax
    mov dword ptr [edi + 4], 0
    add eax, 0x200000
    add edi, 8
    loop map_pages

    # Page tables 
    mov eax, offset pml4_table
    mov cr3, eax

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    # Long Mode
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    lgdt [gdt64_pointer]

    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    # Far jump in 64 Bit
    .byte 0xea
    .long long_mode_start
    .word 0x08

.code64
long_mode_start:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    mov rsp, offset stack_top
    xor rbp, rbp

    call kernel_main

halt:
    cli
    hlt
    jmp halt

.align 8
gdt64:
    .quad 0
    .quad 0x00AF9A000000FFFF
    .quad 0x00CF92000000FFFF
gdt64_end:

gdt64_pointer:
    .word gdt64_end - gdt64 - 1
    .long gdt64

.section .bss
.align 4096
pml4_table:
    .skip 4096
pdpt_table:
    .skip 4096
pd_table:
    .skip 4096

.align 16
stack_bottom:
    .skip 16384
stack_top:
