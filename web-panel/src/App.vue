<template>
  <div class="app">
    <h1>mystic-neuro-console</h1>

    <section>
      <h2>Send Command</h2>
      <label>
        Kind:
        <select v-model="kind">
          <option value="neural_cache_purge">neural_cache_purge</option>
          <option value="context_expand">context_expand</option>
          <option value="session_snapshot">session_snapshot</option>
          <option value="session_recover">session_recover</option>
          <option value="codex_validate">codex_validate</option>
        </select>
      </label>

      <label>
        Role:
        <select v-model="role">
          <option value="admin">admin</option>
          <option value="operator">operator</option>
          <option value="observer">observer</option>
        </select>
      </label>

      <label>
        Session ID (for session_*):
        <input v-model="sessionId" placeholder="session-1" />
      </label>

      <button @click="send">Send</button>
    </section>

    <section>
      <h2>Results</h2>
      <ul>
        <li v-for="r in results" :key="r.command_id">
          <strong>{{ r.command_id }}</strong>: {{ r.message }}
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { connect, onMessage, sendCommand } from './api'

const kind = ref('neural_cache_purge')
const role = ref('admin')
const sessionId = ref('session-1')
const results = ref([])

onMounted(async () => {
  await connect()
  onMessage(data => {
    if (data.type === 'result') {
      results.value.unshift(data)
    } else if (data.type === 'error') {
      results.value.unshift({ command_id: 'error', message: data.message })
    }
  })
})

function send() {
  let scope
  if (kind.value.startsWith('session_')) {
    scope = { kind: 'session', id: sessionId.value }
  } else {
    scope = { kind: 'cluster' }
  }
  sendCommand(kind.value, role.value, scope)
}
</script>

<style scoped>
.app {
  font-family: system-ui, sans-serif;
  padding: 1.5rem;
  max-width: 800px;
  margin: 0 auto;
}
section {
  margin-top: 1.5rem;
}
label {
  display: block;
  margin-bottom: 0.5rem;
}
button {
  margin-top: 0.5rem;
}
</style>
