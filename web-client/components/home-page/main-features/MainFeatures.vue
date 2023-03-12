<script setup>
import IconTune from '@/assets/icons/tune.svg?skipsvgo'
import IconBarChart from '@/assets/icons/bar-chart.svg?skipsvgo'
import IconGroups from '@/assets/icons/groups.svg?skipsvgo'

const { t } = useI18n()
</script>

<template>
  <section>
    <h2>{{ t('heading') }}</h2>

    <div class="cards">
      <HomePageMainFeaturesCard>
        <template #icon>
          <IconTune />
        </template>
        <template #heading>{{ t('cards.controlling.heading') }}</template>
        <template #description>{{
          t('cards.controlling.description')
        }}</template>
      </HomePageMainFeaturesCard>

      <HomePageMainFeaturesCard>
        <template #icon>
          <IconBarChart />
        </template>
        <template #heading>{{ t('cards.analysing.heading') }}</template>
        <template #description>{{ t('cards.analysing.description') }}</template>
      </HomePageMainFeaturesCard>

      <HomePageMainFeaturesCard with-later-badge>
        <template #icon>
          <IconGroups />
        </template>
        <template #heading>{{ t('cards.growingTogether.heading') }}</template>
        <template #description>{{
          t('cards.growingTogether.description')
        }}</template>
      </HomePageMainFeaturesCard>
    </div>
  </section>
</template>

<style lang="scss" scoped>
section {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 2rem;
  padding: 0 0 2rem 0;

  .cards {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 4rem;

    @include xxl-screen {
      flex-direction: row;
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "What we offer?",
    "cards": {
      "controlling": {
        "heading": "Control your garden",
        "description": "The greenhouse operator can remotely monitor monitoring systems and control automated greenhouse processes."
      },
      "analysing": {
        "heading": "Analyse effectiveness",
        "description": "Visual display of readings from temperature, air humidity and soil moisture sensors. Ability to track trends in collected data over a period of time."
      },
      "growingTogether": {
        "heading": "Grow together",
        "description": "Multiple operators can work together on the same greenhouse, and permissions can be configured for each invited user."
      }
    }
  },
  "ru-RU": {
    "heading": "Что мы предлагаем?",
    "cards": {
      "controlling": {
        "heading": "Контролируйте свой огород",
        "description": "Оператор тепличного хозяйства может удаленно следить за системами мониторинга и управлять  автоматизированными процессами теплиц."
      },
      "analysing": {
        "heading": "Анализируйте эффективность",
        "description": "Визуальное отображение показаний с датчиков температуры, влажности воздуха и почвы. Возможность отслеживания динамики собранных данных за период."
      },
      "growingTogether": {
        "heading": "Выращивайте вместе",
        "description": "Возможность совместной работы нескольких операторов с одной теплицей, а также настройка разрешений для каждого приглашенного пользователя."
      }
    }
  }
}
</i18n>
