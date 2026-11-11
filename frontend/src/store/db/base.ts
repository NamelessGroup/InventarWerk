import { ErrorHandler } from '@/errorHandling/ErrorHandler'
import type { AxiosResponse } from 'axios'
import axios from 'axios'

export abstract class BaseDatabaseHandler {
  protected constructor(protected baseURL: string) {}

  public abstract initialize(): Promise<void>

  protected async get<T>(url: URLParts, queryParams?: QueryParameter): Promise<T | undefined> {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .get<T>(this.baseURL + url.join('/'), { params, withCredentials: true })
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data as T
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not get ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  protected async post<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .post<T>(
        this.baseURL + url.join('/'),
        {},
        {
          params,
          withCredentials: true
        }
      )
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not post ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  protected async put<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .put<T>(
        this.baseURL + url.join('/'),
        {},
        {
          params,
          withCredentials: true
        }
      )
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data as T
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not put ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  protected async patch<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .patch<T>(
        this.baseURL + url.join('/'),
        {},
        {
          params,
          withCredentials: true
        }
      )
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not patch ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  protected async delete<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .delete<T>(this.baseURL + url.join('/'), {
        params,
        withCredentials: true
      })
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not delete ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  private wasSuccess(response: AxiosResponse) {
    return response.status >= 200 && response.status < 300
  }

  public static getBaseURL(port: number) {
    return import.meta.env.MODE == 'prod'
      ? `${window.location.origin}/`
      : `http://localhost:${port}/`
  }
}

type URLParts = string[]

type QueryParameter = Record<string, string>
