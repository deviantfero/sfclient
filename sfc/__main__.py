# -*- coding: utf-8 -*-

# import os
# import sys
from . import menu
from . import commons


def main():
    msg = commons.getMsgCode()

    commons.writeMsg(commons.FIFO_PATH, msg + commons.MSG_ARRIVE)
    commons.readMsg(commons.FIFO_PATH, show=False)

    options = {
        1: menu.one,
        2: menu.two,
        3: menu.three,
        4: menu.four,
        5: menu.five,
        6: menu.six,
        7: menu.seven,
        8: menu.eight
    }

    opt = menu.showMenu()
    while(opt != menu.Menu.EXIT):
        options[opt]()
        opt = menu.showMenu()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print(":(")
        menu.eight()
