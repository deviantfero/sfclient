from commons import *
from enum import Enum
from transfer import *

class Menu(Enum):
    NOT_VALID = 0
    SERVER_LS = 1
    SERVER_STATE =2 
    UPLD_FILE = 3
    DWNLD_FILE = 4
    CLIENT_LS = 5
    SET_MODE = 6
    CHANGE_CHUNKSIZE = 7
    EXIT = 8

def one():
    writeMsg(FIFO_PATHW, getMsgCode() + MSG_LS)
    readMsg(FIFO_PATHR)
    input("\nPress enter ")
    os.system('clear')

def two():
    writeMsg(FIFO_PATHW, getMsgCode() + MSG_STATUS)
    readMsg(FIFO_PATHR)
    input("\nPress enter ")
    os.system('clear')

def three():
    os.system('clear')
    files = os.listdir()
    i = 1
    for f in files:
        print( "[{}] {:>48}".format(str(i), f)  )
        i+=1

    opt = 0
    while True:
        try:
            opt = int(input("\nSelect file to transfer:"))
            if(opt > 0 and opt <= len(files)):
                break
        except ValueError:
            continue
        print("Please enter a valid number")
        
    sendFile(files[opt-1])
    os.system('clear')

def four():
    writeMsg(FIFO_PATHW, getMsgCode() + MSG_DOWNLD)
    files = readMsg(FIFO_PATHR, show = False)
    file_ammount = readMsg(FIFO_PATHR, show = False)
    opt = choose_file(files)

    writeMsg(FIFO_PATHW, getMsgCode() + str(opt -1))

    file_info = [readMsg(FIFO_PATHR, show = False)[0], readMsg(FIFO_PATHR, show = False)[0]]

    f = open( file_info[1][:-1], 'wb+')
    f_size = int(file_info[0][:-1])
    readFilePipe(FIFO_PATHR, f_size, f)
    f.close()
                
    
def five():
    os.system('clear')
    files = os.listdir()
    i = 1
    for f in files:
        print( "[{}] {:>48}".format(str(i), f)  )
        i+=1
    input("\nPress enter ")
    os.system('clear')

def six():
    writeMsg(FIFO_PATHW, MSG_LS)
    readMsg(FIFO_PATHR)

def seven():
    global MAX_BUFFER
    writeMsg(FIFO_PATHW, getMsgCode() + MSG_CHUNKSIZE)
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
    writeMsg(FIFO_PATHW, getMsgCode() + str(opt))
    MAX_BUFFER = opt
    os.system('clear')


def eight():
    writeMsg(FIFO_PATHW, getMsgCode() + MSG_EXIT)
    os.system('clear')
    exit(0)
    
def showMenu():
    print('''
[1]  See server content
[2]  See server status
[3]  Upload a file
[4]  Download a file
[5]  See client content
[6]  Set transfer method [pipes]
[7]  Set chunksize [{0}]
[8]  Exit
    '''.format(MAX_BUFFER))
    while True:
        try:
            opt = int(input("\nEnter a number:"))
            if(opt > 0 and opt < 11):
                return opt
        except ValueError:
            continue
        print("Please enter a valid number")
