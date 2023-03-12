<script setup></script>

<template>
  <table>
    <tr class="header">
      <slot name="header" />
    </tr>
    <slot name="content" />
  </table>
</template>

<style lang="scss" scoped>
table {
  width: 100%;
  border: none;
  border-radius: var(--xl-radius);
  border-spacing: 0;
  overflow: hidden;
  table-layout: fixed;
  background: var(--white-original);

  :deep(tr) {
    padding: 0;
    font-size: var(--default-font-size);
    font-weight: 600;
    color: var(--black-original);
  }

  :deep(th),
  :deep(td) {
    position: relative;
    width: 10rem;
    height: 2.25rem;
    padding: 0;
    font-weight: 400;
    text-align: center;

    &::after {
      content: ' ';
      position: absolute;
      display: flex;
      justify-content: flex-end;
      align-items: center;
      right: 0;
      top: 0.675rem;
      width: 0.0625rem;
      height: 1rem;
      border-radius: var(--small-radius);
      background: var(--black-original);

      @include medium-screen {
        top: 0.75rem;
        height: 1.5rem;
      }
    }

    &:last-child::after {
      display: none;
    }

    @include medium-screen {
      height: 2.75rem;
    }
  }

  :deep(tr:not(.header)) {
    &:nth-child(odd) {
      background-color: #e9f0ee;
    }
  }

  .header {
    background: var(--primary-layer-0-background);
    font-size: var(--default-font-size);
    font-weight: 600;
    color: var(--primary-layer-0-color);

    :deep(th) {
      font-weight: 600;

      &::after {
        background: var(--white-original);
      }
    }
  }

  .content {
  }
}

body[data-theme='dark'] table {
  background: var(--black-200);

  :deep(td) {
    color: var(--white-original);

    &::after {
      background: var(--white-original);
    }
  }

  :deep(tr:not(.header)) {
    &:nth-child(odd) {
      background-color: var(--black-300);
    }
  }
}
</style>
