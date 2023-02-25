import { IQuake } from '~~/types'

export const fecthLatestFiftyEvents = async () => {
  const response = await $fetch<{ eventList: Array<IQuake> }>(
    'https://deprem.afad.gov.tr/EventData/GetEventsByFilter',
    {
      method: 'POST',
      body: {
        Skip: 0,
        Take: 50,
        EventSearchFilterList: [],
        SortDescriptor: { field: 'eventDate', dir: 'desc' },
      },
    }
  )
  return response.eventList
}

// export const fetchLatestEventsFromKandilli = async (html: string) => {

// }
