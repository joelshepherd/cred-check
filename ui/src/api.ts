// deno-lint-ignore-file camelcase
import { Ok, Err, Result } from "./deps.ts";

// Status code for now
type Error = number;
type ApiResult<T> = Result<T, Error>;

// Auth

interface LoginRequest {
  username: string;
}

interface SignupRequest {
  name: string;
  username: string;
}

interface Token {
  token: string;
}

export function login(input: LoginRequest): Promise<ApiResult<Token>> {
  return request("post", "login", input);
}

export function signup(input: SignupRequest): Promise<ApiResult<Token>> {
  return request("post", "signup", input);
}

// Opinion

interface CreateOpinion {
  source_id: number;
  position: boolean;
  body: string;
}

export interface Opinion {
  id: number;
  source_id: number;
  user_id: number;
  position: boolean;
  body: string;
}

export function createOpinion(
  input: CreateOpinion
): Promise<ApiResult<Opinion>> {
  return request("post", "opinion", input);
}

// Vote

interface CreateVote {
  opinion_id: number;
}

interface Vote {
  id: number;
  opinion_id: number;
  user_id: number;
}

export function createVote(input: CreateVote): Promise<ApiResult<Vote>> {
  return request("post", "vote", input);
}

// Source

export interface Source {
  id: number;
  title: string;
  canonical_url: string;
}

export interface SourceExt {
  source: Source;
  opinions: Opinion[];
  votes: [number, number];
}

export function findSource(url: string): Promise<ApiResult<SourceExt>> {
  return request("get", `source/${url}`);
}

export function createSource(url: string): Promise<ApiResult<SourceExt>> {
  return request("post", `source`, { url });
}

async function request<T = unknown>(
  method: "get" | "post",
  path: string,
  data?: unknown
): Promise<ApiResult<T>> {
  const authorization = window.localStorage.getItem("token") ?? "";

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
