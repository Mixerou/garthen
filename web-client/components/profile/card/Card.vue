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
      :class="{ 'hide-on-mobile': avatarOnlyOnMobile, hide: avatarOnly }"
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
    transition: var(--fast-transition-duration);
    transition-property: background-color, transform;

    &:hover {
      background: var(--primary-400);
      color: var(--white-original);
    }

    &:active {
      background: var(--primary-500);
      color: var(--white-original-100);
    }

    &.hide-on-mobile {
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

    &.hide {
      width: auto;

      .information {
        display: none;
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
      width: calc(100% - 3.75rem);
      font-size: var(--default-font-size);

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
