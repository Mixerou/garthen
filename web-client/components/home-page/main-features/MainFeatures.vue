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
        <!--      TODO: Text-->
        <template #description>
          Mattis facilisis sagittis a lectus nulla. Augue tellus purus ut vitae
          vitae tempus proin. Nunc ultrices turpis egestas egestas facilisis
          massa turpis lectus.
        </template>
      </HomePageMainFeaturesCard>

      <HomePageMainFeaturesCard>
        <template #icon>
          <IconBarChart />
        </template>
        <template #heading>{{ t('cards.analysing.heading') }}</template>
        <!--      TODO: Text-->
        <template #description>
          At auctor feugiat volutpat vulputate id donec. Vitae euismod risus
          varius scelerisque volutpat ac eget aliquet molestie. Eget tortor amet
          eros viverra consequat sit.
        </template>
      </HomePageMainFeaturesCard>

      <HomePageMainFeaturesCard with-later-badge>
        <template #icon>
          <IconGroups />
        </template>
        <template #heading>{{ t('cards.growingTogether.heading') }}</template>
        <!--      TODO: Text-->
        <template #description>
          Quis aliquam nisl at lacus non. Malesuada nunc leo tortor vel commodo
          nisl et quam fames. Felis duis in convallis arcu enim.
        </template>
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
        "heading": "Control your garden"
      },
      "analysing": {
        "heading": "Analyse your crops"
      },
      "growingTogether": {
        "heading": "Grow together"
      }
    }
  },
  "ru-RU": {
    "heading": "Что мы предлагаем?",
    "cards": {
      "controlling": {
        "heading": "Контролируйте свой огород"
      },
      "analysing": {
        "heading": "Анализируйте свои посевы"
      },
      "growingTogether": {
        "heading": "Выращивайте вместе"
      }
    }
  }
}
</i18n>
