import axios from "axios";
import type { AxiosResponse } from "axios";
import useSWR from "swr";
import useSWRMutation from "swr/mutation";

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

export interface ProblemRank {
  id: number;
  rank: number | null;
  our_score: number | null;
  best_score: number | null;
}

export interface ProblemSetRank {
  updated: string;
  rank: number;
  problems: ProblemRank[];
}

export interface TeamRankResponse {
  scoreboard_last_updated: string;
  total_rank: number;
  lambdaman: ProblemSetRank;
  spaceship: ProblemSetRank;
  threed: ProblemSetRank;
  efficiency: ProblemSetRank;
}

export interface ParsedProblem {
  category: string;
  id: number;
  content: string;
}

export function useProblem(category: string, id: number) {
  const { data, error, isLoading } = useSWR<AxiosResponse<ParsedProblem>>(
    {
      method: "get",
      url: `/problems/${category}/${id}`,
    },
    client,
  );
  return { data: data?.data, error, isLoading };
}

export function useCommunicationSubmit(request: string) {
  const { data, error, trigger, isMutating } = useSWRMutation<
    AxiosResponse<CommunicationLog>
  >(
    {
      method: "post",
      url: "/communicate/submit",
      data: {
        plaintext: request,
      },
    },
    client,
  );
  return { data: data?.data, error, isMutating, trigger };
}

export function useCommunicationLog(id: number) {
  const { data, error, isLoading } = useSWR<AxiosResponse<CommunicationLog>>(
    {
      method: "get",
      url: `/communications/${id}`,
    },
    client,
  );
  return { data: data?.data, error, isLoading };
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

export function useCommunicationsWithExactRequest(
  request: string,
  offset?: number,
  limit?: number,
) {
  const { data, error, isLoading } = useSWR<AxiosResponse<CommunicationLog[]>>(
    {
      method: "get",
      url: "/communications",
      params: {
        offset: offset,
        limit: limit,
        decoded_request: request,
      },
    },
    client,
  );
  return { data: data?.data, error, isLoading };
}

export function useCommunicationsWithRequestPrefix(
  prefix: string,
  offset?: number,
  limit?: number,
) {
  const { data, error, isLoading } = useSWR<AxiosResponse<CommunicationLog[]>>(
    {
      method: "get",
      url: "/communications",
      params: {
        offset: offset,
        limit: limit,
        decoded_request_prefix: prefix,
      },
    },
    client,
  );
  return { data: data?.data, error, isLoading };
}

export function useTeamRank() {
  const { data, error, isLoading } = useSWR<AxiosResponse<TeamRankResponse>>(
    {
      method: "get",
      url: "/team_rank",
    },
    client,
  );
  return { data: data?.data, error, isLoading };
}
