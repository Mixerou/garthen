<script setup></script>

<template>
  <div id="default-layout">
    <Header />
    <main>
      <slot />
    </main>
  </div>
</template>

<style lang="scss" scoped>
#default-layout {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  min-height: 100vh;

  main,
  header {
    @include default-widths;
  }
}
</style>
