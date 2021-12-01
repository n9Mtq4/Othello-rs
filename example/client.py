import socket
import struct


HOST = "localhost"
PORT = 35326


def evaluate_position(host, port, me, enemy, time, params):
    
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        
        # connect to server
        s.connect((host, port))
        
        # pack & send data
        send_buf = struct.pack('!QQHH', me, enemy, time, params)
        s.sendall(send_buf)
        
        # read response
        res_buf = b''
        while len(res_buf) < 3:
            res_buf += s.recv(3)
        
        # return unpacked data (best_move, centidisk_eval)
        return struct.unpack('!Bh', res_buf)


def serialize_params(adj_time, use_book, solve_end_exact, mid_depth, end_depth):
    assert mid_depth in range(0, 64), "invalid range for mid_depth. must be 0-63"
    assert end_depth in range(0, 64), "invalid range for end_depth. must be 0-63"
    p = 0
    p |= end_depth << 0
    p |= mid_depth << 6
    p |= int(solve_end_exact) << 12
    p |= int(use_book) << 13
    p |= int(adj_time) << 14
    return p


if __name__ == '__main__':
    black = 9241636472995985464
    white = 4484490210071479296
    time = 12345
    params = serialize_params(False, True, True, 5, 18)
    print(evaluate_position(HOST, PORT, black, white, time, params))
