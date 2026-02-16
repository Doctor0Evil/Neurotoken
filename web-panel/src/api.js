let socket = null
let handlers = []

export function connect() {
  return new Promise((resolve, reject) => {
    socket = new WebSocket('ws://127.0.0.1:8080')
    socket.onopen = () => resolve()
    socket.onerror = (e) => reject(e)
    socket.onmessage = (event) => {
      const data = JSON.parse(event.data)
      handlers.forEach(h => h(data))
    }
  })
}

export function onMessage(handler) {
  handlers.push(handler)
}

export function sendCommand(kind, role, scope) {
  const payload = {
    type: 'command',
    id: crypto.randomUUID(),
    kind,
    role,
    scope
  }
  socket.send(JSON.stringify(payload))
}
