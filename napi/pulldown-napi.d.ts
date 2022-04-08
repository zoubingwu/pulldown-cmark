import { JsOption } from "./index";

export type UserOptions = Partial<JsOption>;

export function transform(content: string, opts?: UserOptions): string
