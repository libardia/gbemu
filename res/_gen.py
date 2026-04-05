with open('res/rom_dummy.bin', 'bw') as dummy, open('res/rom_ascending.bin', 'bw') as ascending, \
        open('res/rom_too_small.bin', 'bw') as too_small:
    dummy.write(bytes([0]*0x100))
    ascending.write(bytes(range(0x100)))
    too_small.write(bytes(range(0x50)))
