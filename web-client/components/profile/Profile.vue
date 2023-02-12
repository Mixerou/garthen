<script setup></script>

<template>
  <div class="profile">
    <GarthenButton>
      {{ $t('signIn') }}
    </GarthenButton>
  </div>
</template>

<style lang="scss" scoped>
.profile {
  position: relative;
}

.hide {
  opacity: 0;
}
</style>
