import { MessageItem } from "./types.js";

export function formatSize(sizeInB: number): string {
    const sizeInKB = sizeInB / 1024;
    if (sizeInKB >= 1024) {
        return `${(sizeInKB / 1024).toFixed(2)} MB`;
    }
    return `${sizeInKB.toFixed(2)} KB`;
}

export function calculateTotalSize(messages: MessageItem[]): number {
    return messages.reduce((sum, item) => sum + item.size, 0);
}
