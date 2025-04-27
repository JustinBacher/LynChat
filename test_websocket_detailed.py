import asyncio
import websockets
import json
import time

async def test_websocket():
    uri = "ws://localhost:8083/ws/chat"
    print(f"Connecting to {uri}...")
    
    try:
        async with websockets.connect(uri) as websocket:
            print("Connected successfully!")
            
            # Send a message
            message = {
                "message": "Hello, how are you?",
                "timestamp": "2023-04-27T00:00:00Z"
            }
            
            print(f"Sending message: {json.dumps(message)}")
            await websocket.send(json.dumps(message))
            print("Message sent, waiting for response...")
            
            # Wait for the response with a timeout
            try:
                response = await asyncio.wait_for(websocket.recv(), timeout=10.0)
                print(f"Raw response received: {response}")
                
                # Try to parse as JSON
                try:
                    parsed = json.loads(response)
                    print(f"Parsed JSON response: {json.dumps(parsed, indent=2)}")
                    
                    # Check if it's an echo
                    if isinstance(response, str) and response.startswith("Echo: "):
                        print("ISSUE DETECTED: Response starts with 'Echo: ', indicating an echo server behavior")
                except json.JSONDecodeError:
                    print("Response is not valid JSON")
                    
                    # Check if it's an echo with the "Echo: " prefix
                    if response.startswith("Echo: "):
                        original_msg = response[6:]  # Remove "Echo: " prefix
                        try:
                            original_json = json.loads(original_msg)
                            print(f"ISSUE DETECTED: The response is 'Echo: ' + the original message: {json.dumps(original_json, indent=2)}")
                        except json.JSONDecodeError:
                            print("Could not parse the echo content as JSON")
            except asyncio.TimeoutError:
                print("Timeout waiting for response")
    except Exception as e:
        print(f"Error: {e}")

asyncio.run(test_websocket())
