import os
from commons import *

def readFilePipe(fifo_path, data_size, out_file):
    while not os.path.exists(fifo_path):
        time.sleep(1)

    count = 0
    with open(fifo_path, 'rb') as fdata:
        while count < data_size:
            if data_size - count < MAX_BUFFER:
                chunk = fdata.read(data_size - count)
            else:
                chunk = fdata.read(MAX_BUFFER)
            out_file.write(chunk)
            count= out_file.tell()

def sendFile(f):
    writeMsg(FIFO_PATHW, getMsgCode() + MSG_UPLD)
    f_size = os.path.getsize(f)
    writeMsg(FIFO_PATHW, getMsgCode() + str(f_size))
    writeMsg(FIFO_PATHW, getMsgCode() + f)
    
    count = 0
    with open(FIFO_PATHW, 'wb') as fdata, open(f, 'rb') as fout:
        while count < f_size:
            if f_size - count < MAX_BUFFER:
                chunk = fout.read(f_size - count)
                count = f_size
            else:
                chunk = fout.read(MAX_BUFFER)
                count += MAX_BUFFER
            fdata.write(chunk)


def choose_file(files):
    opt = 0
    while(opt < 1 or opt > len(files)):
        os.system('clear')
        for f in files:
            print(f, end="")
        try:
            opt = int(input("\nInput a file number:"))
        except ValueError:
            continue
    return opt
