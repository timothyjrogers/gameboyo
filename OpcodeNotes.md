# Opcode Notes

## 0x00 NOP
* Size: 1 byte
* Time: 4 clocks

No operation instruction, just eats clocks.

## 0x01 LD BC, d16
* Size: 3 bytes
* Time: 12 clocks

Loads 16-bit immediate value into register BC.

## 0x02 LD (BC), A
* Size: 1 byte
* Time: 8 clocks

Stores the contents of register A (8 bits) in the memory location stored in register BC.

## 0x03 INC BC
* Size: 1 byte
* Time: 4 bytes
* Flags: ZNH

Increments register BC.

## 0x04 INC B
* Size: 1 byte
* Time: 4 bytes
* Flags: ZNH

Increments register B.

## 0x05 DEC B
* Size: 1 byte
* Time: 4 bytes

Decrements register B;

## 0x06 LD B, d8
* Size: 2
* Time: 8 clocks

Loads 8-bit immediate value into register B.

## 0x07 RLCA
* Size: 1
* Time: 4 clocks
* Flags: ZNHC

Rotates register A left. Bit 7 rotates to bit 0 and copies to CY (carry flag).

## 0x08 LD (a16), SP
* Size: 3
* Time: 20 clocks

Stores the lower byte of SP at address a16, uppper byte of SP at a16+1;

## 0x09 ADD HL, BC
* Size: 1
* Time: 8 clocks
* Flags: NHC

Adds contents of register HL and BC, stores result in HL.

## 0x0A LD A, (BC)
* Size: 1
* Time: 8 clocks

Loads the 8-bit contents of memory from the address in BC into register A.

## 0x0B DEC BC
* Size 1
* Time: 4 clocks

Decrements register BC.

## 0x0C INC C
* Size: 1
* Time: 4 clocks
* Flags: ZNH

Increments register C.

## 0x0D DEC C
* Size: 1
* Time: 4 clocks
* Flags: ZNH

Decrements register C.

## 0x0E LD C, d8
* Size: 2
* Time: 8 clocks

Loads 8-bit immediate into register C.

## 0x0F RRCA
* Size: 1
* Time: 4 clocks
* Flags: ZNHC

Rotates register A to the right. Bit 0 is rotated to bit 7 and copied to CY.

## 0x10 STOP 0
* Size: 2
* Time: 4 clocks

Enters STOP mode.

## 0x11 LD DE, d16
* Size: 3 bytes
* Time: 12 clocks

Loads 16-bit immediate value into register DE.

## 0x12 LD (DE), A
* Size: 1 byte
* Time: 8 clocks

Stores the contents of register A (8 bits) in the memory location stored in register DE.

## 0x13 INC DE
* Size: 1 byte
* Time: 8 clocks

Increments register DE.

## 0x14 INC D
* Size: 1
* Time: 4 clocks
* Flags: ZNH

Increments register D.

## 0x15 DEC D
* Size: 1
* Time: 4 clocks
* Flags: ZNH

Decrements register D.

## 0x16 LD D, d8
* Size: 2
* Time: 8 clocks

Loads 8-bit immediate value into register D.

## 0x17 RLA
* Size: 1 byte
* Time: 4 clocks
* Flags: ZNHC

Rotates register A left. Bit 7 copies to CY (carry bit).

## 0x18 JR r8
* Size: 2
* Time: 12 clocks

Jumps between -127 to +129 from the current PC.

## 0x19 ADD HL, DE
* Size: 1 bytes
* Time: 8 clocks
* Flags: NHC

Adds HL and DE and stores the result in HL.

## 0x1A LD A, (DE)
* Size: 1 bytes
* Time: 8 clocks

Loads the 8-bit contents of the address stored in DE into register A.

## 0x1B DEC DE
* Size: 1
* Time: 8 clocks

Decrements register DE.

## 0x1C INC E
* Size: 1
* Time: 4 clocks
* Flags: ZNH

Increments register E.

## 0x1D DEC E
* Size: 1
* Time: 4 clocks
* Flags: ZNH

Decrements register E.

## 0x1E LD E, d8
* Size: 2
* Time: 8 clocks

Loads 8-bit immediate value into register E.

## 0x1F RRA
* Size: 1
* Time: 4 clocks
* Flags: ZNHC

Rotates register A right. Bit 0 is loaded into CY.

## 0x20 JR NZ, r8