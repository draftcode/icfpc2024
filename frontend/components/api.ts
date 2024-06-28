import axios from "axios";
import type { AxiosResponse } from "axios";
import useSWR from "swr";

const client = axios.create({
  baseURL: "https://icfp-api.badalloc.com/",
});

export interface CommunicationLog {
  id: number;
  created: string;
  request: string;
  response: string;
  decoded_request: string;
  decoded_response: string;
}

export function useCommunications(offset?: number, limit?: number) {
  const { data, error, isLoading } = useSWR<AxiosResponse<CommunicationLog[]>>(
    {
      method: "get",
      url: "/communications",
      params: {
        offset: offset,
        limit: limit,
      },
    },
    client,
  );
  return { data: data?.data, error, isLoading };
}
