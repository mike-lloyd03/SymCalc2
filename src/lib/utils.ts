import type { HistoryItem } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export async function getHistory() {
	let result: HistoryItem[] = await invoke("get_history");
	return result;
}

export function scrollToBottom(div: HTMLDivElement) {
	div.scroll({ top: div.scrollHeight, behavior: "smooth" });
}
