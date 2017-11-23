# -*- coding: utf-8 -*-

import os, sys
from commons import *
from menu import *
from transfer import *

def main():
    msg = getMsgCode()

    writeMsg(FIFO_PATH, msg+ MSG_ARRIVE)
    readMsg(FIFO_PATH, show=False)
    
    options = {
        1 : one,
        2 : two,
        3 : three,
        4 : four,
        5 : five,
        6 : six,
        7 : seven,
        8 : eight
    }
    
    opt = showMenu()
    while(opt != Menu.EXIT):
        options[opt]()
        opt = showMenu()
        
if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print(":(")
        eight()
