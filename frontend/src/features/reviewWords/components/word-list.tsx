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

export default function WordList() {
  const [words, setWords] = useState<Word[]>([]);
  const [error, setError] = useState<string | null>(null);

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

  const deleteWord = async (id: number) => {
    try {
      await invoke("delete_word", { id });
      setWords(words.filter((word) => word.id !== id));
    } catch (error) {
      console.error("Failed to delete word:", error);
      setError(`Error: ${error}`);
    }
  };

  return (
    <div className="p-4">
      {error && <div className="text-red-500 mb-4">{error}</div>}

      <div>
        <h2 className="text-lg font-bold mb-2">単語リスト ({words.length})</h2>
        {words.length > 0 ? (
          <div className="space-y-3">
            {words.map((word) => (
              <Card key={word.id} className="my-1">
                <CardHeader>
                  <CardTitle>
                    {word.vocabulary} (id: {word.id})
                  </CardTitle>
                  <CardDescription>カテゴリ: {word.category}</CardDescription>
                </CardHeader>
                <CardContent>
                  <p>意味：{word.meaning} </p>
                  <p>翻訳：{word.translate}</p>
                </CardContent>
                <CardFooter>
                  <div>
                    {word.example && (
                      <div className="text-sm italic">例: {word.example}</div>
                    )}
                    <Button
                      variant={"destructive"}
                      onClick={() => deleteWord(word.id)}
                      className="m-1"
                    >
                      削除
                    </Button>
                    <Button variant={"ghost"}>
                      <Link href={`/update/${word.id}`}>更新</Link>
                    </Button>
                  </div>
                </CardFooter>
              </Card>
            ))}
          </div>
        ) : (
          <p>
            単語がありません。
            <Link href={"/add"}> 単語を追加してください。</Link>
          </p>
        )}
      </div>
    </div>
  );
}
