<script setup>
import dateFormat, { i18n as dateFormatI18n } from 'dateformat'
import {
  CategoryScale,
  Chart,
  Colors,
  LinearScale,
  LineController,
  LineElement,
  PointElement,
  Tooltip,
} from 'chart.js'

const props = defineProps({
  devices: {
    type: Array,
    required: true,
  },
  range: {
    type: Number,
    required: true,
  },
})

const i18n = useI18n()
const constants = useConstantsStore()
const dataStore = useDataStore()

const chart = ref(null)
let chartInstance = null

let labels = []
let datasets = []

const numberOfUpdates = ref(0)

let resizeTimeout = null

const isMobile = ref(false)
const screenChecker = ref(null)
const screenCheckerObserver = new ResizeObserver(event => {
  isMobile.value = screenChecker.value.clientWidth === 0
})

const render = () => {
  numberOfUpdates.value += 1
  chartInstance = new Chart(chart.value, {
    type: 'line',
    data: { labels, datasets },
    options: {
      animations: {
        y: { duration: 0 },
      },
      responsive: true,
      interaction: { intersect: false, mode: 'index' },
      scales: {
        x: {
          grid: { color: '#ffffff40' },
          ticks: {
            color: '#ffffff',
            font: {
              family: `'Nunito Sans', sans-serif`,
              size: 14,
              weight: 400,
            },
          },
        },
        y: {
          grid: { color: '#ffffff40' },
          ticks: {
            color: '#ffffff',
            font: {
              family: `'Nunito Sans', sans-serif`,
              size: 14,
              weight: 400,
            },
          },
        },
      },
      plugins: {
        tooltip: {
          backgroundColor: '#1e3034',
          titleFont: {
            family: `'Nunito Sans', sans-serif`,
            size: 16,
            weight: 600,
          },
          titleAlign: 'center',
          bodyFont: {
            family: `'Nunito Sans', sans-serif`,
            size: 14,
            weight: 400,
          },
          bodySpacing: 4,
          padding: { x: 12, y: 8 },
          caretPadding: 4,
          cornerRadius: 12,
          boxWidth: 1,
          boxPadding: 4,
        },
      },
    },
  })
}

const onResize = () => {
  clearTimeout(resizeTimeout)

  resizeTimeout = setTimeout(() => {
    if (chartInstance !== null) chartInstance.destroy()

    render()
  }, 1e3)
}

onMounted(() => {
  Chart.register(
    CategoryScale,
    Colors,
    LineController,
    LineElement,
    LinearScale,
    PointElement,
    Tooltip
  )
  render()
  screenCheckerObserver.observe(screenChecker.value)
  window.addEventListener('resize', onResize)

  watchEffect(() => {
    const ranges = constants.DEVICE_RECORDS_TIMESTAMP_RANGES
    let localLabels = []
    let localDatasets = []

    dateFormatI18n.monthNames = [
      i18n.t('monthShortNames.january'),
      i18n.t('monthShortNames.february'),
      i18n.t('monthShortNames.march'),
      i18n.t('monthShortNames.april'),
      i18n.t('monthShortNames.may'),
      i18n.t('monthShortNames.june'),
      i18n.t('monthShortNames.july'),
      i18n.t('monthShortNames.august'),
      i18n.t('monthShortNames.september'),
      i18n.t('monthShortNames.october'),
      i18n.t('monthShortNames.november'),
      i18n.t('monthShortNames.december'),
    ]

    for (const device of [...props.devices].sort((a, b) =>
      a['external_id'] < b['external_id'] ? 1 : -1
    )) {
      const deviceRecords =
        dataStore.deviceRecordsAverage[props.range] === undefined
          ? undefined
          : dataStore.deviceRecordsAverage[props.range][device.id]
      let records = []

      if (deviceRecords === undefined || deviceRecords.records === undefined)
        return

      const sortedDeviceRecords = [...deviceRecords.records].sort((a, b) =>
        a.range[0] > b.range[0] ? 1 : -1
      )

      for (const record of sortedDeviceRecords) {
        records.push(record.data)
      }

      if (localLabels.length === 0) {
        for (const record of sortedDeviceRecords) {
          const firstDate = new Date(record.range[0] * 1e3)
          const secondDate = new Date(record.range[1] * 1e3)

          if (
            isMobile.value &&
            props.range === ranges.today
          )
            localLabels.push(dateFormat(secondDate, 'H'))
          else if (
            props.range ===
            ranges.lastThreeMoths
          )
            localLabels.push(dateFormat(secondDate, 'mmm'))
          else if (isMobile.value)
            localLabels.push(
              ` ${dateFormat(firstDate, 'dd')} - ${dateFormat(
                secondDate,
                'dd'
              )} `
            )
          else if (props.range === ranges.today)
            localLabels.push(dateFormat(secondDate, 'H:00'))
          else if (props.range === ranges.week)
            localLabels.push(dateFormat(secondDate, 'mmm dd'))
          else if (props.range === ranges.month)
            localLabels.push(
              `${dateFormat(firstDate, 'mmm dd')} - ${dateFormat(
                secondDate,
                'mmm dd'
              )}`
            )
          else if (props.range === ranges.lastMonth)
            localLabels.push(
              `${dateFormat(firstDate, 'mmm dd')} - ${dateFormat(
                secondDate,
                'mmm dd'
              )}`
            )
          else if (props.range === ranges.monthBeforeLast)
            localLabels.push(
              `${dateFormat(firstDate, 'mmm dd')} - ${dateFormat(
                secondDate,
                'mmm dd'
              )}`
            )
        }
      }

      localDatasets.push({
        label:
          device.name === null
            ? i18n.t(`defaultDeviceNames.${device.kind}`, {
                externalId: device['external_id'],
              })
            : device.name,
        data: records,
        borderColor: device.color,
        backgroundColor: device.color,
        cubicInterpolationMode: 'monotone',
        tension: 0.4,
      })
    }

    labels = localLabels
    datasets = localDatasets
    chartInstance.data.labels = labels
    chartInstance.data.datasets = datasets
    numberOfUpdates.value += 1

    chartInstance.update()
  })
})

onBeforeUnmount(() => {
  screenCheckerObserver.unobserve(screenChecker.value)
  window.removeEventListener('resize', onResize)
  setTimeout(() => chartInstance.destroy(), 1e3)
})
</script>

<template>
  <div class="chart">
    <div ref="screenChecker" class="screen-checker" />
    <canvas ref="chart" />
  </div>
</template>

<style lang="scss" scoped>
.chart {
  display: flex;
  justify-content: center;
  width: 100%;
  height: 100%;

  @include medium-screen {
    align-items: center;
  }

  .screen-checker {
    position: fixed;
    width: 0;
    height: 0;
    pointer-events: none;

    @include medium-screen {
      width: 1rem;
      height: 1rem;
    }
  }

  canvas {
    padding: 0.25rem 0.5rem;
    border-radius: var(--large-radius);
    background: var(--primary-layer-0-background);

    @include medium-screen {
      padding: 1rem 1.5rem;
    }
  }
}
</style>
