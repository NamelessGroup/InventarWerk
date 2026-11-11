import { ErrorHandler } from "@/errorHandling/ErrorHandler"
import axios from "axios"

const LOWER_BOUND_SIZE = 100 * 1000
const UPPER_BOUND_SIZE = 200 * 1000

export async function processList(list: unknown[], url: string, key: string) {
  let i = 0
  while (i < list.length) {
    const currentTransferList = {
      [key]: [] as unknown[]
    }
    while (
      getJsonSizeInBytes(currentTransferList) < LOWER_BOUND_SIZE &&
      i < list.length
    ) {
      currentTransferList[key].push(list[i])
      i++
    }
    if (getJsonSizeInBytes(currentTransferList) > UPPER_BOUND_SIZE) {
      currentTransferList.presets.splice(-1)
      i--
    }
    pushPresetListToServer(currentTransferList, url)
  }
}

async function pushPresetListToServer(data: unknown, url: string) {
  const response = await axios
    .put<unknown>(
      url,
      JSON.stringify(data),
      {
        withCredentials: true
      }
    )
    .then((response) => response)
    .catch((error) => error.response)
  if (response && response.status >= 200 && response.status < 300) {
    return
  } else {
    ErrorHandler.getInstance().registerError(
      new Error(
        `Could put external presets to Server due to: ${response.status} ${response.statusText}`
      )
    )
  }
}

// not used anymore, but should be kept to debug changes in the future
// eslint-disable-next-line @typescript-eslint/no-unused-vars
async function pushPresetToServer(data: unknown, url: string) {
  const response = await axios
    .put<unknown>(
      url,
      JSON.stringify(data),
      {
        withCredentials: true
      }
    )
    .then((response) => response)
    .catch((error) => error.response)
  if (response && response.status == 500) {
    await new Promise((resolve) => setTimeout(resolve, 1000))
    await pushPresetToServer(data, url)
  }
  if (response && response.status >= 200 && response.status < 300) {
    return
  } else {
    ErrorHandler.getInstance().registerError(
      new Error(
        `Could put external presets to Server due to: ${response.status} ${response.statusText}`
      )
    )
  }
}

function getJsonSizeInBytes(data: unknown): number {
  const jsonString = JSON.stringify(data)
  return new TextEncoder().encode(jsonString).length
}
