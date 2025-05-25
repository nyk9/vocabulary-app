"use client";

import { useState, useEffect } from "react";
import { Word } from "@/types/word";
import { invoke } from "@tauri-apps/api/core";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Loader2 } from "lucide-react";

export default function ApiTest() {
  const [words, setWords] = useState<Word[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [recommendations, setRecommendations] = useState<any>(null);
  const [loading, setLoading] = useState<boolean>(false);

  useEffect(() => {
    const handleWord = async () => {
      try {
        const wordList = await invoke<Word[]>("get_words");
        setWords(wordList || []);
      } catch (error) {
        console.error("Failed to get words:", error);
        setError(`Error: ${error}`);
      }
    };
    handleWord();
  }, []);

  const handleGetSuggestions = async () => {
    // if (words.length === 0) {
    //   setError("単語リストが空です。単語を追加してください。");
    //   return;
    // }

    setLoading(true);
    try {
      const response = await fetch(
        "https://vocabulary-app-coral.vercel.app/api/suggestion-word",
        {
          method: "POST", // GETではなくPOSTに変更
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ vocabulary: words }),
        },
      );

      if (!response.ok) {
        throw new Error(`API responded with status: ${response.status}`);
      }

      const data = await response.json();
      setRecommendations(data);
      setError(null);
    } catch (err) {
      console.error("Failed to get suggestions:", err);
      setError(`推薦の取得に失敗しました: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container mx-auto py-8">
      <h1 className="text-2xl font-bold mb-6">単語推薦APIテスト</h1>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          {error}
        </div>
      )}

      <Card className="mb-6">
        <CardHeader>
          <CardTitle>現在の単語リスト ({words.length}単語)</CardTitle>
          <CardDescription>Tauriから取得した単語リスト</CardDescription>
        </CardHeader>
        <CardContent>
          {words.length > 0 ? (
            <ul className="space-y-2">
              {words.slice(0, 5).map((word, index) => (
                <li key={index} className="p-2 rounded">
                  <strong>{word.vocabulary}</strong> - {word.translate} (
                  {word.category || "未分類"})
                </li>
              ))}
              {words.length > 5 && (
                <li className="">...その他 {words.length - 5} 単語</li>
              )}
            </ul>
          ) : (
            <p>単語が見つかりません</p>
          )}
        </CardContent>
        <CardFooter>
          <Button onClick={handleGetSuggestions} disabled={loading}>
            {loading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                推薦取得中...
              </>
            ) : (
              "単語推薦を取得"
            )}
          </Button>
        </CardFooter>
      </Card>

      {recommendations && (
        <Card>
          <CardHeader>
            <CardTitle>おすすめの単語</CardTitle>
            <CardDescription>AIが推薦する次に学ぶべき単語</CardDescription>
          </CardHeader>
          <CardContent>
            {recommendations.content && recommendations.content[0]?.text ? (
              <div className="whitespace-pre-wrap">
                {recommendations.content[0].text}
              </div>
            ) : (
              <pre className="bg-gray-100 p-4 rounded overflow-auto">
                {JSON.stringify(recommendations, null, 2)}
              </pre>
            )}
          </CardContent>
        </Card>
      )}

      <div className="mt-6">
        <Link href="/">
          <Button variant="outline">ホームに戻る</Button>
        </Link>
      </div>
    </div>
  );
}
