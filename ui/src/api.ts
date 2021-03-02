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

interface TokenReply {
  token: string;
}

export function login(input: LoginRequest): Promise<ApiResult<TokenReply>> {
  return request("post", "login", input);
}

export function signup(input: SignupRequest): Promise<ApiResult<TokenReply>> {
  return request("post", "signup", input);
}

// Opinion

interface CreateOpinion {
  source_id: number;
  position: boolean;
  body: string;
}

export interface OpinionReply {
  id: number;
  position: boolean;
  body: string;
}

export function createOpinion(
  input: CreateOpinion
): Promise<ApiResult<OpinionReply>> {
  return request("post", "opinion", input);
}

// Vote
interface CreateVote {
  opinion_id: number;
}

export function createVote(input: CreateVote): Promise<ApiResult<void>> {
  return request("post", "vote", input);
}

// Source

export interface SourceReply {
  id: number;
  title: string;
  canonical_url: string;
  opinions: OpinionReply[];
  votes: [number, number];
}

export function findSource(url: string): Promise<ApiResult<SourceReply>> {
  return request("get", `source/${url}`);
}

export function createSource(url: string): Promise<ApiResult<SourceReply>> {
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

  const reply =
    res.headers.get("content-type") === "application/json"
      ? await res.json()
      : void 0;

  return Ok(reply);
}
