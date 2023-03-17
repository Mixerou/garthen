<script setup>
const props = defineProps({
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  greenhouseName: {
    type: String,
    required: true,
  },
  greenhouseToken: {
    type: String,
    required: true,
  },
})
const emit = defineEmits(['update:greenhouseName', 'update:greenhouseToken'])

const { t } = useI18n()

const computedName = computed({
  get: () => {
    return props.greenhouseName
  },
  set: value => {
    if (props.disabled) return
    emit('update:greenhouseName', value)
  },
})

const computedToken = computed({
  get: () => {
    return props.greenhouseToken
  },
  set: value => {
    if (props.disabled) return
    emit('update:greenhouseToken', value)
  },
})
</script>

<template>
  <section>
    <h5>{{ t('heading') }}</h5>

    <div class="groups">
      <div class="group">
        <p class="key">{{ $t('greenhouseName') }}</p>
        <GarthenInput
          v-model:text="computedName"
          :disabled="disabled"
          max-length="32"
        />
      </div>
      <div class="group">
        <p class="key">{{ $t('token') }}</p>
        <GarthenInput
          v-model:text="computedToken"
          :disabled="disabled"
          type="password"
          max-length="32"
        />
      </div>
    </div>
  </section>
</template>

<style lang="scss" scoped></style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "General"
  },
  "ru-RU": {
    "heading": "Общее"
  }
}
</i18n>
