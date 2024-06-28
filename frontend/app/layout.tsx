import type { Metadata } from "next";
import "./globals.css";
import ContentsContainer from "./ContentsContainer";

export const metadata: Metadata = {
  title: "icfpc2024",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="h-screen">
        <div className="max-w-[70rem] mx-auto h-screen flex flex-col justify-between">
          <ContentsContainer>{children}</ContentsContainer>
        </div>
      </body>
    </html>
  );
}
