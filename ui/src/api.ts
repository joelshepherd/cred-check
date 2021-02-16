import { Ok, Err, Result } from "./deps.ts";

// Status code for now
type Error = number;
type ApiResult<T> = Result<T, Error>;

interface AddOpinionRequest {
  source_id: number;
  position: boolean;
  body: string;
}

export function addOpinion(
  input: AddOpinionRequest
): Promise<ApiResult<Opinion>> {
  return request("post", "opinion", input);
}

interface CreateSupporter {
  opinion_id: number;
}

interface Supporter {
  id: number;
  opinion_id: number;
  user_id: number;
}

export function addSupporter(
  input: CreateSupporter
): Promise<ApiResult<Supporter>> {
  return request("post", "supporter", input);
}

export interface Source {
  id: number;
  title: string;
  url: string;
}

export interface Opinion {
  id: number;
  source_id: number;
  user_id: number;
  position: boolean;
  body: string;
}

export interface FindSource {
  source: Source;
  opinions: Opinion[];
  votes: [number, number];
}

export function findSource(url: string): Promise<ApiResult<FindSource>> {
  return request("get", `source/${url}`);
}

export function createSource(url: string): Promise<ApiResult<FindSource>> {
  return request("post", `source`, { url });
}

async function request<T = unknown>(
  method: "get" | "post",
  path: string,
  data?: object
): Promise<ApiResult<T>> {
  const authorization = localStorage.getItem("token") ?? "";

  const res = await fetch(`http://localhost:8080/${path}`, {
    method,
    body: data ? JSON.stringify(data) : undefined,
    mode: "cors",
    headers: { authorization, "content-type": "application/json" },
    credentials: "include",
  });

  if (!res.ok) return Err(res.status);

  return Ok(await res.json());
}
