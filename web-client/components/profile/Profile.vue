<script setup>
defineProps({
  hideCardWhenCardModalOpened: {
    type: Boolean,
    required: false,
    default: false,
  },
  avatarOnlyInCardOnMobile: {
    type: Boolean,
    required: false,
    default: false,
  },
  avatarOnly: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const user = useUserStore()

const isAuthModalOpened = ref(false)
const isCardModalOpened = ref(false)

const changeAuthModalState = isOpen => {
  setTimeout(() => (isAuthModalOpened.value = isOpen), 10)
}

const openCardModal = () => {
  if (user.email === '' || user.username === '') return

  isCardModalOpened.value = true
}
</script>

<template>
  <div class="profile">
    <GarthenModal
      v-if="isAuthModalOpened"
      close-on-click-outside
      @close="isAuthModalOpened = false"
    >
      <ProfileAuth />
    </GarthenModal>
    <GarthenModal
      v-if="isCardModalOpened"
      close-on-click-outside
      @close="isCardModalOpened = false"
    >
      <ProfileCardModal />
    </GarthenModal>
    <transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
      <ProfileCard
        v-if="user.isLoggedIn"
        class="fast-transition"
        :class="{ hide: hideCardWhenCardModalOpened && isCardModalOpened }"
        :avatar-only-on-mobile="avatarOnlyInCardOnMobile"
        :avatar-only="avatarOnly"
        @click="openCardModal"
      />
      <GarthenButton
        v-else
        class="fast-transition"
        @click="changeAuthModalState(true)"
      >
        {{ $t('signIn') }}
      </GarthenButton>
    </transition>
  </div>
</template>

<style lang="scss" scoped>
.profile {
  position: relative;
}

.fast-transition {
  transition: var(--fast-transition-duration);
}

.hide {
  opacity: 0;
}
</style>
