export type ExpressionInput = {
	text: string;
	cursorPos: number;
};

export type HistoryItem = {
	expression: string;
	result: string;
};
