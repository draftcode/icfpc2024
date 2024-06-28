import * as fs from "fs";
import { NextRequest, NextResponse } from "next/server";

const BS = require("biwascheme");

// interface Request {
//   program: string;
// }

interface Response {
  result?: string | number;
  error?: string;
}

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const program = searchParams.get("program");

  let result;
  try {
    result = BS.run(program);
  } catch (e) {
    return NextResponse.json({ error: (e as Error).message } as Response);
  }
  return NextResponse.json({ result } as Response);
}
