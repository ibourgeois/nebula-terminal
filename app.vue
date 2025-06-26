<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-900 via-purple-900 to-indigo-900">
    <div class="container mx-auto px-4 py-8">
      <div class="text-center">
        <h1 class="text-4xl font-bold text-white mb-4">Nebula Terminal</h1>
        <p class="text-blue-200 mb-8">A modern terminal application built with Tauri and NuxtJS</p>
        <div class="bg-white/10 backdrop-blur-sm rounded-lg p-6 shadow-xl">
          <NuxtRouteAnnouncer />
          <NuxtWelcome />
        </div>
      </div>
    </div>
  </div>
</template>
