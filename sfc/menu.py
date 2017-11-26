from enum import Enum
import os
from . import transfer
from . import commons
from .commons import MAX_BUFFER, SIGNAL


class Menu(Enum):
    NOT_VALID = 0
    SERVER_LS = 1
    SERVER_STATE = 2
    UPLD_FILE = 3
    DWNLD_FILE = 4
    CLIENT_LS = 5
    SET_MODE = 6
    CHANGE_CHUNKSIZE = 7
    EXIT = 8


def one():
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + commons.MSG_LS)
    commons.readMsg(commons.FIFO_PATHR)
    input("\nPress enter ")
    os.system('clear')


def two():
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + commons.MSG_STATUS)
    commons.readMsg(commons.FIFO_PATHR)
    input("\nPress enter ")
    os.system('clear')


def three():
    os.system('clear')
    files = os.listdir()
    i = 1
    for f in files:
        print("[{}] {:>48}".format(str(i), f))
        i += 1

    opt = 0
    while True:
        try:
            opt = int(input("\nSelect file to transfer:"))
            if(opt > 0 and opt <= len(files)):
                break
        except ValueError:
            continue
        print("Please enter a valid number")
    if(SIGNAL == 0):
        transfer.sendFile(files[opt-1])
    else:
        transfer.sendFileQ(files[opt-1])
    os.system('clear')


def four():
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + commons.MSG_DOWNLD)
    files = commons.readMsg(commons.FIFO_PATHR, show=False)
    opt = transfer.choose_file(files)
    commons.writeMsg(commons.FIFO_PATHW, commons.getMsgCode() + str(opt - 1))

    file_info = [commons.readMsg(commons.FIFO_PATHR, show=False)[0],
                 commons.readMsg(commons.FIFO_PATHR, show=False)[0]]

    f = open(file_info[1][:-1], 'wb+')
    f_size = int(file_info[0][:-1])
    if(SIGNAL == 0):
        transfer.readFilePipe(commons.FIFO_PATHR, f_size, f)
    else:
        transfer.readFileQueue(commons.FIFO_PATHQ, f_size, f)
    f.close()


def five():
    os.system('clear')
    files = os.listdir()
    i = 1
    for f in files:
        print("[{}] {:>48}".format(str(i), f))
        i += 1
    input("\nPress enter ")
    os.system('clear')


def six():
    global SIGNAL
    print(
        '''
        [0] Pipes
        [1] Queues'''
    )
    opt = 0
    while True:
        try:
            opt = int(input("Enter a method number:"))
            if(opt == 0 or opt == 1):
                break
        except ValueError:
            continue
        print("Please enter a valid number")
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + commons.MSG_METHOD)
    commons.writeMsg(commons.FIFO_PATHW, commons.getMsgCode() + str(opt))
    os.system('clear')
    SIGNAL = opt


def seven():
    global MAX_BUFFER
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + commons.MSG_CHUNKSIZE)
    os.system('clear')
    opt = 0
    while True:
        try:
            opt = int(input("Enter new chunk size:"))
            if(opt > 0):
                break
        except ValueError:
            continue
        print("Please enter a valid number")
    commons.writeMsg(commons.FIFO_PATHW, commons.getMsgCode() + str(opt))
    MAX_BUFFER = opt
    os.system('clear')


def eight():
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + commons.MSG_EXIT)
    os.system('clear')
    exit(0)


def showMenu():
    method = ['pipes', 'queues']
    print('''
[1]  See server content
[2]  See server status
[3]  Upload a file
[4]  Download a file
[5]  See client content
[6]  Set transfer method [{1}]
[7]  Set chunksize [{0}]
[8]  Exit
    '''.format(MAX_BUFFER, method[SIGNAL]))
    while True:
        try:
            opt = int(input("\nEnter a number:"))
            if(opt > 0 and opt < 11):
                return opt
        except ValueError:
            continue
        print("Please enter a valid number")
