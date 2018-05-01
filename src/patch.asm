; You can use this to patch the game's code to call into the Rom Hack's code
0x802ff5d4:
; lis r3, 0x8045 Use 804504A0 as ArenaLow
u32 0x3c608045

0x80006458:
bl game_loop
bl 0x80022e74 ; fapGm_Execute__Fv

0x802c8d9c:
b draw
