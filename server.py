import socketserver
import threading
from urllib import response

class RequestHandler(socketserver.BaseRequestHandler):

    def handle(self):
        data = str(self.request.recv(1024), 'ascii')
        cur_thread = threading.current_thread()
        response = bytes(f"{cur_thread.name}: {data}", 'ascii')
        self.request.sendall(response)

def main():
    host, port = "localhost", 9999

    with socketserver.ThreadingTCPServer((host, port), RequestHandler) as server:
        server_thread = threading.Thread(target=server.serve_forever)
        server_thread.daemon = True
        server_thread.start()
        print("Hello")
        server_thread.join()


if __name__ == "__main__":
    main()
    exit(0)
