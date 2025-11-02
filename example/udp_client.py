import socket

# Create a UDP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

# Send data
server_address = ("127.0.0.1", 7)
message = b"Hello, UDP Server!"

try:
    print(f"sending {message}")
    sent = sock.sendto(message, server_address)

    # Receive response
    print("waiting to receive")
    data, server = sock.recvfrom(4096)
    print(f"received {data.decode()}")

finally:
    print("closing socket")
    sock.close()
