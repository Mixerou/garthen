<script setup>
import ImageGardeningLight from '@/assets/images/gardening-light.svg?skipsvgo'
import ImageGardeningDark from '@/assets/images/gardening-dark.svg?skipsvgo'
import ImageMessagingFunLight from '@/assets/images/messaging-fun-light.svg?skipsvgo'
import ImageMessagingFunDark from '@/assets/images/messaging-fun-dark.svg?skipsvgo'

const { t } = useI18n()
</script>

<template>
  <div class="sections">
    <HomePageAdditionalFeaturesSection>
      <template #heading>{{ t('sections.manyGreenhouses.heading') }}</template>
      <template #description>
        {{ t('sections.manyGreenhouses.description') }}
      </template>
      <template #image>
        <ImageGardeningLight class="light" />
        <ImageGardeningDark class="dark" />
      </template>
    </HomePageAdditionalFeaturesSection>

    <HomePageAdditionalFeaturesSection with-later-badge reverse>
      <template #heading>{{ t('sections.quickNotice.heading') }}</template>
      <template #description>
        {{ t('sections.quickNotice.description') }}
      </template>
      <template #image>
        <ImageMessagingFunLight class="messaging-fun-image light" />
        <ImageMessagingFunDark class="messaging-fun-image dark" />
      </template>
    </HomePageAdditionalFeaturesSection>
  </div>
</template>

<style lang="scss" scoped>
.sections {
  display: flex;
  flex-direction: column;

  .messaging-fun-image {
    padding-bottom: 2.5rem;

    @include medium-screen {
      padding-bottom: 4.125rem;
    }
  }
}

svg.dark {
  display: none;
}

body[data-theme='dark'] {
  svg.light {
    display: none;
  }

  svg.dark {
    display: initial;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "sections": {
      "manyGreenhouses": {
        "heading": "A lot of greenhouses?",
        "description": "The Garthen remote monitoring and control system allows up to 15 greenhouses to be monitored simultaneously."
      },
      "quickNotice": {
        "heading": "Quick notice",
        "description": "There is no need for manual monitoring. Telegram bot will notify about critical sensor readings and emergency situations. It will give recommendations on how to manage the situation."
      }
    }
  },
  "ru-RU": {
    "sections": {
      "manyGreenhouses": {
        "heading": "Много теплиц?",
        "description": "Система удаленного мониторинга и управления Garthen позволяет одновременно отслеживать до 15 теплиц."
      },
      "quickNotice": {
        "heading": "Быстрое оповещение",
        "description": "Нет необходимости «ручного» мониторинга. Telegram-бот оповестит о критических показаниях датчиков и аварийных ситуациях. Выдаст рекомендации по регулированию ситуации."
      }
    }
  }
}
</i18n>
