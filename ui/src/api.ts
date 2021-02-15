interface AddOpinionRequest {
  source_id: number;
  position: boolean;
  body: string;
}

export function addOpinion(input: AddOpinionRequest): Promise<Opinion> {
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

export function addSupporter(input: CreateSupporter): Promise<Supporter> {
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

export interface SourceResponse {
  source: Source;
  opinions: Opinion[];
  votes: [number, number];
}

export function findSource(url: string): Promise<SourceResponse> {
  return request("get", `source/${url}`);
}

async function request<T>(
  method: "get" | "post",
  path: string,
  data?: object
): Promise<T> {
  const authorization = localStorage.getItem("token") ?? "";

  const res = await fetch(`http://localhost:8080/${path}`, {
    method,
    body: data ? JSON.stringify(data) : undefined,
    mode: "cors",
    headers: { authorization, "content-type": "application/json" },
    credentials: "include",
  });

  return res.json();
}
