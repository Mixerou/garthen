<script setup>
import dateFormat, { i18n as dateFormatI18n } from 'dateformat'

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

const { t } = useI18n()
const i18n = useI18n()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()

const parsedData = computed(() => {
  const timestamps = constants.DEVICE_RECORDS_TIMESTAMP_RANGES
  let records = []
  let periods = []

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
    i18n.t('monthNames.january'),
    i18n.t('monthNames.february'),
    i18n.t('monthNames.march'),
    i18n.t('monthNames.april'),
    i18n.t('monthNames.may'),
    i18n.t('monthNames.june'),
    i18n.t('monthNames.july'),
    i18n.t('monthNames.august'),
    i18n.t('monthNames.september'),
    i18n.t('monthNames.october'),
    i18n.t('monthNames.november'),
    i18n.t('monthNames.december'),
  ]

  for (const device of props.devices) {
    const deviceRecords =
      dataStore.deviceRecordsAverage[props.range] === undefined
        ? undefined
        : dataStore.deviceRecordsAverage[props.range][device.id]
    let localRecords = []

    if (deviceRecords === undefined || deviceRecords.records === undefined)
      return

    const sortedDeviceRecords = [...deviceRecords.records].sort((a, b) =>
      a.range[0] < b.range[0] ? 1 : -1
    )

    for (const record of sortedDeviceRecords) {
      localRecords.push(record.data)
    }

    records.push(localRecords)

    if (periods.length === 0) {
      for (const record of sortedDeviceRecords) {
        const firstDate = new Date(record.range[0] * 1e3)
        const secondDate = new Date(record.range[1] * 1e3)

        if (props.range === timestamps.today)
          periods.push(dateFormat(secondDate, 'H:00'))
        else if (props.range === timestamps.week)
          periods.push(dateFormat(secondDate, 'mmmm dd'))
        else if (
          props.range === timestamps.month ||
          props.range === timestamps.lastMonth ||
          props.range === timestamps.monthBeforeLast
        )
          periods.push(
            `${dateFormat(firstDate, 'mmm dd')} - ${dateFormat(
              secondDate,
              'mmm dd'
            )}`
          )
        else if (props.range === timestamps.lastThreeMoths)
          periods.push(dateFormat(secondDate, 'mmmm'))
      }
    }
  }

  let result = []

  periods.forEach((period, index) => {
    let localRecords = []

    records.forEach(iteratedRecords =>
      localRecords.push(iteratedRecords[index])
    )
    result.push({ period, records: localRecords })
  })

  return result
})
</script>

<template>
  <GarthenTable>
    <template #header>
      <th>{{ t('period') }}</th>
      <th v-for="device in devices" :key="`table-header-${device.id}`">
        {{
          device.name === null
            ? $t(`defaultDeviceNames.${device.kind}`, {
                externalId: device['external_id'],
              })
            : device.name
        }}
      </th>
    </template>

    <template #content>
      <tr v-for="data in parsedData" :key="`table-row-${data.period}`">
        <td>{{ data.period }}</td>
        <td
          v-for="(record, index) in data.records"
          :key="`table-row-${data.period}-${index}`"
        >
          {{ record || '---' }}
        </td>
      </tr>
    </template>

    <!--    <template #content>-->
    <!--      <tr-->
    <!--        v-for="(period, periodIndex) in periods"-->
    <!--        :key="`table-row-${periodIndex}`"-->
    <!--      >-->
    <!--        <td>{{ period }}</td>-->
    <!--        <td-->
    <!--          v-for="(device, deviceIndex) in devices"-->
    <!--          :key="`table-row-${periodIndex}-${deviceIndex}`"-->
    <!--        >-->
    <!--          {{ parsedRecords[deviceIndex][periodIndex] || '-&#45;&#45;' }}-->
    <!--        </td>-->
    <!--      </tr>-->
    <!--    </template>-->

    <!--    <template #content>-->
    <!--      <tr-->
    <!--        v-for="(data, dataIndex) in parsedRecords"-->
    <!--        :key="`table-row-${dataIndex}`"-->
    <!--      >-->
    <!--        <td>{{ periods[dataIndex] }}</td>-->
    <!--        <td-->
    <!--          v-for="(device, deviceIndex) in devices"-->
    <!--          :key="`table-row-${dataIndex}-${deviceIndex}`"-->
    <!--        >-->
    <!--          {{-->
    <!--            `${deviceIndex} - ${dataIndex}` +-->
    <!--            parsedRecords[deviceIndex][dataIndex]-->
    <!--          }}-->
    <!--        </td>-->
    <!--      </tr>-->
    <!--    </template>-->
  </GarthenTable>
</template>

<style lang="scss" scoped>
table {
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "period": "Period"
  },
  "ru-RU": {
    "period": "Период"
  }
}
</i18n>
