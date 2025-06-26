<template>
  <div class="min-h-screen bg-terminal-bg text-terminal-fg font-terminal">
    <!-- Terminal Window -->
    <div class="h-screen flex flex-col">
      <!-- Terminal Title Bar -->
      <div class="bg-nord-1 border-b border-nord-2 flex items-center justify-between px-4 py-2">
        <div class="flex items-center space-x-2">
          <!-- Window Controls -->
          <div class="flex space-x-2">
            <div class="w-3 h-3 rounded-full bg-nord-11"></div>
            <div class="w-3 h-3 rounded-full bg-nord-13"></div>
            <div class="w-3 h-3 rounded-full bg-nord-14"></div>
          </div>
          <!-- Title -->
          <div class="ml-4 text-nord-4 text-sm font-medium">
            nebula-terminal — zsh
          </div>
        </div>
        <!-- Window Actions -->
        <div class="flex items-center space-x-2 text-nord-4">
          <button class="hover:text-nord-6 transition-colors">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
            </svg>
          </button>
        </div>
      </div>

      <!-- Terminal Content Area -->
      <div class="flex-1 bg-terminal-bg p-4 overflow-hidden">
        <div class="h-full flex flex-col">
          <!-- Terminal Output -->
          <div class="flex-1 overflow-y-auto space-y-1">
            <!-- Welcome Message -->
            <div class="text-nord-8 font-bold">
              ╭─────────────────────────────────────────────────────────────╮
            </div>
            <div class="text-nord-8 font-bold text-center">
              │                    Nebula Terminal v0.1.0                    │
            </div>
            <div class="text-nord-8 font-bold">
              ╰─────────────────────────────────────────────────────────────╯
            </div>
            
            <div class="text-nord-4 text-sm mt-4">
              Welcome to Nebula Terminal - A modern cross-platform terminal built with Tauri, NuxtJS, and TailwindCSS.
            </div>
            
            <div class="text-nord-4 text-sm mt-2">
              Type <span class="text-nord-8 font-mono">help</span> to see available commands.
            </div>

            <!-- Sample Terminal Output -->
            <div class="mt-6 space-y-1">
              <div class="flex items-center">
                <span class="text-nord-14 font-mono mr-2">$</span>
                <span class="text-nord-6 font-mono">ls -la</span>
              </div>
              <div class="text-nord-4 text-sm ml-4 font-mono">
                total 8<br>
                drwxr-xr-x  15 user  staff   480 Dec 26 01:15 .<br>
                drwxr-xr-x   3 user  staff    96 Dec 26 01:15 ..<br>
                -rw-r--r--   1 user  staff   614 Dec 26 01:15 .gitignore<br>
                -rw-r--r--   1 user  staff  1086 Dec 26 01:15 LICENSE<br>
                -rw-r--r--   1 user  staff  8192 Dec 26 01:15 README.md<br>
                drwxr-xr-x   8 user  staff   256 Dec 26 01:15 src-tauri<br>
                -rw-r--r--   1 user  staff  1024 Dec 26 01:15 package.json
              </div>
            </div>

            <div class="mt-4 space-y-1">
              <div class="flex items-center">
                <span class="text-nord-14 font-mono mr-2">$</span>
                <span class="text-nord-6 font-mono">pwd</span>
              </div>
              <div class="text-nord-4 text-sm ml-4 font-mono">
                /Users/user/code/github/ibourgeois/nebula
              </div>
            </div>

            <div class="mt-4 space-y-1">
              <div class="flex items-center">
                <span class="text-nord-14 font-mono mr-2">$</span>
                <span class="text-nord-6 font-mono">git status</span>
              </div>
              <div class="text-nord-4 text-sm ml-4 font-mono">
                On branch feature/terminal-ui-nord-theme<br>
                Changes not staged for commit:<br>
                &nbsp;&nbsp;(use "git add &lt;file&gt;..." to update what will be committed)<br>
                &nbsp;&nbsp;(use "git restore &lt;file&gt;..." to discard changes in working directory)<br>
                &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;modified:&nbsp;&nbsp;app.vue<br>
                &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;modified:&nbsp;&nbsp;tailwind.config.js<br>
                <br>
                no changes added to commit (use "git add" and/or "git commit -a")
              </div>
            </div>
          </div>

          <!-- Command Input Area -->
          <div class="mt-4 border-t border-nord-2 pt-4">
            <div class="flex items-center">
              <span class="text-nord-14 font-mono mr-2 text-terminal-lg">$</span>
              <div class="flex-1 relative">
                <input 
                  type="text" 
                  class="w-full bg-transparent text-terminal-fg font-mono text-terminal-lg outline-none border-none focus:ring-0"
                  placeholder="Enter command..."
                  v-model="commandInput"
                  @keyup.enter="executeCommand"
                  ref="commandInput"
                />
                <!-- Cursor -->
                <div class="absolute top-0 left-0 w-0.5 h-5 bg-terminal-cursor animate-pulse" 
                     :style="{ left: cursorPosition + 'px' }"></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, nextTick } from 'vue'

const commandInput = ref('')
const cursorPosition = ref(0)

const executeCommand = () => {
  // TODO: Implement command execution
  console.log('Executing command:', commandInput.value)
  commandInput.value = ''
  cursorPosition.value = 0
}

onMounted(() => {
  nextTick(() => {
    // Focus the command input
    if (commandInput.value) {
      commandInput.value.focus()
    }
  })
})
</script>

<style scoped>
/* Custom scrollbar for terminal */
.overflow-y-auto::-webkit-scrollbar {
  width: 8px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #2E3440;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: #4C566A;
  border-radius: 4px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: #5E81AC;
}

/* Terminal cursor animation */
@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.animate-pulse {
  animation: blink 1s infinite;
}
</style>
