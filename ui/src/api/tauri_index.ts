import { invoke } from "@tauri-apps/api/core";
import { Annotation, RelationGroup, PdfAnnotations } from "../context";

export interface Token {
    x: number;
    y: number;
    height: number;
    width: number;
    text: string;
}

export interface Page {
    index: number;
    width: number;
    height: number;
}

export interface PageTokens {
    page: Page;
    tokens: Token[];
}

export interface Label {
    text: string;
    color: string;
}

export interface PaperStatus {
    sha: string;
    name: string;
    annotations: number;
    relations: number;
    finished: boolean;
    junk: boolean;
    comments: string;
    completedAt?: Date;
}

export interface Allocation {
    papers: PaperStatus[];
    hasAllocatedPapers: boolean;
}

// ---------------------------------------------------------------------------

function docURL(sha: string): string {
    return `/api/doc/${sha}`;
}

export function pdfURL(sha: string): string {
    return `${docURL(sha)}/pdf`;
}

// ---------------------------------------------------------------------------

export async function getTokens(sha: string): Promise<PageTokens[]> {
  console.info("getTokens", sha);
  return invoke("get_tokens", { sha });
}

export async function getLabels(): Promise<Label[]> {
  console.info("getLabels");
  return invoke("get_labels");
}

export async function getRelations(): Promise<Label[]> {
  console.info("getRelations");
  return invoke("get_relations");
}

export async function setPdfComment(sha: string, comments: string) {
  console.info("setPdfComment", sha, comments);
  return invoke("set_pdf_comment", { sha, comments });
}

export async function setPdfFinished(sha: string, finished: boolean) {
  console.info("setPdfFinished", sha, finished);
  return invoke("set_pdf_finished", { sha, finished });
}

export async function setPdfJunk(sha: string, junk: boolean) {
  console.info("setPdfJunk", sha, junk);
  return invoke("set_pdf_junk", { sha, junk });
}

export async function getAllocatedPaperStatus(): Promise<Allocation> {
  console.info("getAllocatedPaperStatus");
  return invoke("get_allocated_paper_status");
}

export function saveAnnotations(
  sha: string,
  pdfAnnotations: PdfAnnotations
): Promise<any> {
  console.info("saveAnnotations", sha, pdfAnnotations);
  return invoke("save_annotations", {
    sha,
    annotations: pdfAnnotations.annotations,
    relations: pdfAnnotations.relations,
  });
}

export async function getAnnotations(sha: string): Promise<PdfAnnotations> {
  console.info("getAnnotations", sha);
  const ann = await invoke<{
    annotations: unknown[];
    relations: unknown[];
  }>("get_annotations", { sha });

  return new PdfAnnotations(
    ann.annotations.map((a: any) => Annotation.fromObject(a)),
    ann.relations.map((r: any) => RelationGroup.fromObject(r))
  );
}
