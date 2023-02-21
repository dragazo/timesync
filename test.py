import websocket
import time
import sys

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f'usage: {sys.argv[0]} [url]', file = sys.stderr)
        sys.exit(1)
    url = sys.argv[1]

    def send_request(ws):
        ws.send(str(round(time.time() * 1000)))

    def on_message(ws, message):
        print('message', message, flush = True)
        send_request(ws)

    def on_error(ws, error):
        print('error', error, flush = True)

    def on_close(ws, close_status_code, close_msg):
        print('close', close_status_code, close_msg)

    def on_open(ws):
        print('open')
        send_request(ws)

    websocket.enableTrace(False)
    ws = websocket.WebSocketApp(url, on_open = on_open, on_message = on_message, on_error = on_error, on_close = on_close)
    ws.run_forever()
