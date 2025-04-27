import asyncio
import websockets
import json

async def test_websocket():
    uri = "ws://localhost:8083/ws/chat"
    async with websockets.connect(uri) as websocket:
        # Send a message
        message = {
            "message": "Hello, how are you?",
            "timestamp": "2023-04-27T00:00:00Z"
        }
        await websocket.send(json.dumps(message))
        print(f"Sent: {message}")
        
        # Receive the response
        response = await websocket.recv()
        print(f"Received: {response}")
        
        try:
            parsed = json.loads(response)
            print(f"Parsed JSON: {json.dumps(parsed, indent=2)}")
        except json.JSONDecodeError:
            print("Response is not valid JSON")

asyncio.run(test_websocket())
