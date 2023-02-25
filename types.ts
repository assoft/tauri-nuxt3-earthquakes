export interface IQuake {
  id: number
  eventDate: Date | string
  longitude?: number
  latitude?: number
  magnitude: number
  magnitudeType?: string
  location: string
  depth: number
  rms?: number
  erh?: number
  erz?: number
  gap?: number
  eaeventId?: number
  crustModelId?: number
  eventTypeId?: number
  eventType?: string
  magnitudeDescription?: string | null
  formattedDate?: string | null
  depthDescription?: string | null
  refId?: number
  province?: string | null
  district?: string | null
  typeName?: string | null
  typeNameEng?: string | null
  magnitudeName?: string | null
  magnitudeNameEng?: string | null
  timeName?: string | null
  timeNameEng?: string | null
  momentTensor?: string | null
  distanceInformation?: string | null
  bulletins?: string | null
  moments?: string | null
  amplitudes?: string | null
}

export type PIQuake = Pick<IQuake, 'id' | 'eventDate' | 'depth' | 'magnitude' | 'location'>
