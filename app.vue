<template>
  <div class="min-h-screen font-mono" style="background-color: #2E3440; color: #ECEFF4;">
    <!-- Terminal Window -->
    <div class="h-screen flex flex-col">
      <!-- Terminal Content Area -->
      <div class="flex-1 p-4 overflow-hidden" style="background-color: #2E3440;">
        <div class="h-full flex flex-col">
          <!-- Terminal Output -->
          <div class="flex-1 overflow-y-auto space-y-1">
            <!-- Sample Terminal Output -->
            <div class="space-y-1">
              <div class="flex items-center">
                <span class="font-mono mr-2" style="color: #A3BE8C;">$</span>
                <span class="font-mono" style="color: #ECEFF4;">ls -la</span>
              </div>
              <div class="text-sm ml-4 font-mono" style="color: #D8DEE9;">
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
                <span class="font-mono mr-2" style="color: #A3BE8C;">$</span>
                <span class="font-mono" style="color: #ECEFF4;">pwd</span>
              </div>
              <div class="text-sm ml-4 font-mono" style="color: #D8DEE9;">
                /Users/user/code/github/ibourgeois/nebula
              </div>
            </div>

            <div class="mt-4 space-y-1">
              <div class="flex items-center">
                <span class="font-mono mr-2" style="color: #A3BE8C;">$</span>
                <span class="font-mono" style="color: #ECEFF4;">git status</span>
              </div>
              <div class="text-sm ml-4 font-mono" style="color: #D8DEE9;">
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
          <div class="mt-4 pt-4" style="border-top: 1px solid #434C5E;">
            <div class="flex items-center">
              <span class="font-mono mr-2 text-lg" style="color: #A3BE8C;">$</span>
              <div class="flex-1 relative">
                <input 
                  type="text" 
                  class="w-full bg-transparent font-mono text-lg outline-none border-none focus:ring-0"
                  style="color: #ECEFF4;"
                  placeholder="Enter command..."
                  v-model="commandInput"
                  @keyup.enter="executeCommand"
                  ref="commandInput"
                />
                <!-- Cursor -->
                <div class="absolute top-0 left-0 w-0.5 h-5 animate-pulse" 
                     style="background-color: #D8DEE9; left: 0px;"></div>
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

/* Ensure input placeholder uses Nord colors */
input::placeholder {
  color: #4C566A;
}
</style>
