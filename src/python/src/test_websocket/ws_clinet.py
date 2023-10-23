import websocket

def on_message(ws, message):
    print(f"Received message from server: {message}")

def on_error(ws, error):
    print(f"Error: {error}")

def on_close(ws, close_status_code, close_msg):
    print("WebSocket connection closed")

if __name__ == "__main__":
    # 请替换以下 URL 为你的 WebSocket 服务器的 URL
    websocket_url = "ws://localhost:8888/websocket"
    ws = websocket.WebSocketApp(websocket_url, on_message=on_message, on_error=on_error, on_close=on_close)

    # 启动 WebSocket 连接
    ws.run_forever()

    # 循环发送消息给服务器
    while True:
        message = input("Enter a message to send to the server (or 'exit' to quit): ")
        if message == "exit":
            break
        else:
            ws.send(message)

