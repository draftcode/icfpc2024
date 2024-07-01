import axios from "axios";
import type { AxiosResponse } from "axios";
import useSWR, { Key } from "swr";
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

export interface ThreedSimulationResult {
  board: string;
  output: number | null;
  score: number;
  error: string | null;
}

export interface ThreedResolveResult {
  board: string;
  error: string | null;
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

interface ThreedSimulationArg {
  board: string;
  valA: number;
  valB: number;
  turns: number;
}

export function use3DSimulation() {
  const fetcher = async (
    url: string,
    { arg: { board, valA, valB, turns } }: { arg: ThreedSimulationArg },
  ) => {
    return await client({
      method: "post",
      url: url,
      data: {
        board: board,
        val_a: valA,
        val_b: valB,
        turns: turns,
      },
    });
  };
  const { data, error, trigger, isMutating } = useSWRMutation<
    AxiosResponse<ThreedSimulationResult>,
    any,
    Key,
    ThreedSimulationArg
  >("/simulation/3d", fetcher);
  return { data: data?.data, error, isMutating, trigger };
}

export function use3DResolve() {
  const fetcher = async (
    url: string,
    { arg: { board } }: { arg: { board: string } },
  ) => {
    return await client({
      method: "post",
      url: url,
      data: {
        board: board,
      },
    });
  };
  const { data, error, trigger, isMutating } = useSWRMutation<
    AxiosResponse<ThreedResolveResult>,
    any,
    Key,
    ThreedSimulationArg
  >("/simulation/3d/resolve", fetcher);
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

export function useSolutions(
  category: string,
  id: number,
  offset?: number,
  limit?: number,
) {
  const { data, error, isLoading } = useSWR<AxiosResponse<CommunicationLog[]>>(
    {
      method: "get",
      url: `/solutions/${category}/${id}`,
      params: {
        offset: offset,
        limit: limit,
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
