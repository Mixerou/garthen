<script setup>
defineProps({
  avatarOnlyOnMobile: {
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
</script>

<template>
  <div
    class="container"
    :class="{ loading: user.email === '' || user.username === '' }"
  >
    <GarthenLoader
      class="loader"
      :stop="user.email !== '' && user.username !== ''"
    />
    <div
      class="card"
      :class="{ 'fold-on-mobile': avatarOnlyOnMobile, fold: avatarOnly }"
    >
      <ProfileAvatar class="avatar" :username="user.username" />
      <div class="information">
        <p class="username">{{ user.username }}</p>
        <p class="email">{{ user.email }}</p>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.container {
  position: relative;
  border-radius: var(--large-radius);
  overflow: hidden;
  transition: var(--fast-transition-duration);

  &.loading {
    background: var(--primary-100);

    .loader {
      opacity: 1;
      transform: translate(-50%, -50%);
    }

    .card {
      opacity: 0;
      transform: scale(0.95);
      pointer-events: none;
    }
  }

  &.hide .card {
    cursor: unset;
  }

  .loader {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 1.5rem;
    height: 1.5rem;
    opacity: 0;
    transform: translate(-50%, -50%) scale(1.25);
    pointer-events: none;
    transition: var(--fast-transition-duration);
  }

  .card {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 16rem;
    height: 3.75rem;
    padding: 0 1rem;
    background: transparent;
    cursor: pointer;
    transition: width var(--default-transition),
      background-color var(--fast-transition-duration),
      transform var(--fast-transition-duration);

    &:hover {
      background: var(--primary-400);
      color: var(--white-original);
    }

    &:active {
      background: var(--primary-500);
      color: var(--white-original-100);
    }

    &.fold-on-mobile {
      width: auto;

      @include medium-screen {
        width: 16rem;
      }

      .information {
        display: none;

        @include medium-screen {
          display: flex;
        }
      }
    }

    &.fold {
      width: calc(4.75rem - 1rem * 2);

      .information {
        width: 0;
        margin-right: calc(-100% - 1rem);
        opacity: 0;
      }
    }

    .avatar {
      width: 2.75rem;
      height: 2.75rem;
      border-radius: var(--large-radius);
    }

    .information {
      display: flex;
      flex-direction: column;
      width: 12.25rem;
      font-size: var(--default-font-size);
      transition: var(--default-transition);
      transition-property: width, margin-right, opacity;

      .username {
        overflow: hidden;
        white-space: nowrap;
        font-weight: 600;
        text-overflow: ellipsis;
      }

      .email {
        opacity: 0.75;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
      }
    }
  }
}
</style>
