import * as fs from "fs";
import { NextRequest, NextResponse } from "next/server";

// interface Request {
//   id: number;
// }

interface Response {
  data: string;
}

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const id = searchParams.get("id");

  const data = await fs.promises.readFile(
    `../problems/lambdaman/${id}.txt`,
    "utf8"
  );

  return NextResponse.json({ data } as Response);
}
