export type ExpressionInput = {
	text: string;
	cursorPos: number;
};

export type HistoryItem = {
	id?: number;
	equation: string;
	solution: number;
};
