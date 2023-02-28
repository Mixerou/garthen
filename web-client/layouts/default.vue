<script setup></script>

<template>
  <div id="default-layout">
    <div class="layout-content">
      <Header />
      <main>
        <slot />
      </main>
      <Footer />
    </div>
  </div>
</template>

<style lang="scss" scoped>
#default-layout {
  width: 100%;

  .layout-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    height: fit-content;

    main,
    header {
      @include default-widths;
    }
  }
}
</style>
