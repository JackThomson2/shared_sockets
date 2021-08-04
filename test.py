import testmulti
from multiprocessing import Process

counting = 0


def cloned_func(caller):
    global counting
    counting += 1
    print(f"printting from thread {caller} count {counting}")


def fun(name, socket, func):
    print(f"Adding {name}")
    socket.tell_info()

    while True:
        socket.listen(name, func)


def main():
    core_socket = testmulti.SocketHeld("0.0.0.0:9009")

    for i in range(3):
        copied = core_socket.try_clone()
        p = Process(
            target=fun,
            args=(f"Item {i + 1}", copied, cloned_func),
        )
        p.start()


if __name__ == "__main__":
    main()
