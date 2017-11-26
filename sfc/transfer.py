import os
import time
import posix_ipc
from . import commons


def readFilePipe(fifo_path, data_size, out_file):
    while not os.path.exists(fifo_path):
        time.sleep(0.1)

    count = 0
    with open(fifo_path, 'rb') as fdata:
        while count < data_size:
            if data_size - count < commons.MAX_BUFFER:
                chunk = fdata.read(data_size - count)
            else:
                chunk = fdata.read(commons.MAX_BUFFER)
            out_file.write(chunk)
            count = out_file.tell()


def readFileQueue(fifo_path, data_size, out_file):
    # while not os.path.exists(fifo_path):
    #     time.sleep(1)

    time.sleep(0.1)

    # flags = posix_ipc.O_CREX
    mq = posix_ipc.MessageQueue(name=fifo_path,
                                flags=posix_ipc.O_CREAT,
                                max_message_size=commons.commons.MAX_BUFFER,
                                write=False)
    print("Done!")

    count = 0
    while count < data_size:
        if data_size - count < commons.MAX_BUFFER:
            chunkC, _ = mq.receive()
            chunk = chunkC[:(data_size - count)]
        else:
            chunk, _ = mq.receive()
        out_file.write(chunk)
        count = out_file.tell()


def sendFile(f):
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + commons.MSG_UPLD)
    f_size = os.path.getsize(f)
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + str(f_size))
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + f)

    count = 0
    with open(commons.FIFO_PATHW, 'wb') as fdata, open(f, 'rb') as fout:
        while count < f_size:
            if f_size - count < commons.MAX_BUFFER:
                chunk = fout.read(f_size - count)
                count = f_size
            else:
                chunk = fout.read(commons.MAX_BUFFER)
                count += commons.MAX_BUFFER
            fdata.write(chunk)


def sendFileQ(f):
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + commons.MSG_UPLD)
    f_size = os.path.getsize(f)
    commons.writeMsg(commons.FIFO_PATHW,
                     commons.getMsgCode() + str(f_size))
    commons.writeMsg(commons.FIFO_PATHW, commons.getMsgCode() + f)

    mq = posix_ipc.MessageQueue(name=commons.FIFO_PATHQ,
                                flags=posix_ipc.O_CREAT,
                                max_message_size=commons.MAX_BUFFER,
                                read=False)
    count = 0
    with open(f, 'rb') as fout:
        while count < f_size:
            if f_size - count < commons.MAX_BUFFER:
                chunk = fout.read(f_size - count)
                count = f_size
            else:
                chunk = fout.read(commons.MAX_BUFFER)
                count += commons.MAX_BUFFER
            mq.send(chunk)


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
