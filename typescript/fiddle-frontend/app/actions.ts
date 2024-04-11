'use server'
import { kv } from "@vercel/kv";
import { nanoid } from 'nanoid'
import { revalidatePath } from "next/cache";

export type EditorFile = {
  path: string;
  content: string;

}

export async function createUrl(editorFiles: EditorFile[]): Promise<string> {
  const urlId = nanoid()
  console.log(editorFiles)

  const user = await kv.set(urlId, editorFiles);
  return urlId;
}

export async function updateUrl(urlId: string, editorFiles: EditorFile[]): Promise<void> {
  console.log("setting files", editorFiles);
  const user = await kv.set(urlId, editorFiles);
  revalidatePath(`/`);
}

export async function loadUrl(urlId: string): Promise<EditorFile[]> {
  const user = await kv.get(urlId);
  // console.log("loading files", user);

  return user as EditorFile[];
}