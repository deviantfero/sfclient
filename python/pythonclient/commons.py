import os, sys, math, codecs
import os.path
import time

SIGNAL = 0
SENDER = 1
MAX_TOKENS = 10
DFT_TRIES  = 7
WAIT_TIME  = 10000
MAX_BUFFER = 4096
MSG_CHUNKSIZE = "chunksize"
MSG_ENCRYPT   = "encrypt"
MSG_COMPRESS  = "compress"
MSG_METHOD    = "method"
MSG_LS        = "ls"
MSG_STATUS    = "status"
MSG_UPLD      = "upload"
MSG_DONE      = "done"
MSG_UPLD_E    = "eupload"
MSG_UPLD_C    = "cupload"
MSG_DOWNLD    = "download"
MSG_EXIT      = "bye"
MSG_ARRIVE    = "hello"

FIFO_PATH = '/tmp/sfs'
FIFO_PATHW = '/tmp/sfc' + str(os.getpid()) + 'w'
FIFO_PATHR = '/tmp/sfc' + str(os.getpid()) + 'r'
FIFO_PATHQ = '/qsfc' + str(os.getpid())

def readMsg(fifo_path, rm_vals=True, show=True):
    while not os.path.exists(fifo_path):
        time.sleep(0.1)

    res = []
    os.system('clear')
    with codecs.open(fifo_path, "r",encoding='utf-8', errors='ignore') as fdata:
        if(rm_vals):
            fdata.read(7)
        for line in fdata:
            res.append(line)
            if(show):
                print(line, end="")
    return res

def writeMsg(fifo_path, input):
    pipe = open(fifo_path, 'w')
    pipe.write(input+input[len(input)-1])

def mkfifoF(tpath):
     try:
         os.mkfifo(tpath, 0o666)
     except OSError as e:
         raise
     else:
         return tpath

def getMsgCode():
    msg = ""
    pid = os.getpid()
    size_pid = math.ceil(math.log(pid, 10))

    for i in range(7-size_pid):
        msg+="0"
    msg+=str(pid)
    return msg
