"use client";

import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { useToast } from "@/hooks/use-toast";

interface Word {
  id: number;
  vocabulary: string;
  meaning: string;
  translate: string;
  category: string;
  part_of_speech: string[];
  example?: string;
}

interface GradingResponse {
  score: number;
  feedback: string;
}

export default function QuizPage() {
  const [words, setWords] = useState<Word[]>([]);
  const [currentWord, setCurrentWord] = useState<Word | null>(null);
  const [userAnswer, setUserAnswer] = useState<string>("");
  const [gradingResult, setGradingResult] = useState<GradingResponse | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const { toast } = useToast();

  const fetchWords = useCallback(async () => {
    try {
      const fetchedWords = (await invoke("get_words")) as Word[];
      if (fetchedWords.length === 0) {
        setError("単語が登録されていません。まず単語を追加してください。");
        return;
      }
      setWords(fetchedWords);
      const randomIndex = Math.floor(Math.random() * fetchedWords.length);
      setCurrentWord(fetchedWords[randomIndex]);
    } catch (err) {
      console.error("Failed to fetch words:", err);
      setError("単語の読み込みに失敗しました。");
    }
  }, []);

  useEffect(() => {
    fetchWords();
  }, [fetchWords]);

  const handleGradeAnswer = async () => {
    if (!currentWord || !userAnswer.trim()) {
      toast({
        title: "エラー",
        description: "解答を入力してください。",
        variant: "destructive",
      });
      return;
    }

    setIsLoading(true);
    setError(null);
    setGradingResult(null);

    try {
      const result = (await invoke("grade_answer", {
        vocabulary: currentWord.vocabulary,
        meaning: currentWord.meaning,
        userAnswer: userAnswer,
      })) as GradingResponse;
      setGradingResult(result);

      // クイズ実施回数を記録
      await invoke("add_date", {
        date: { // このオブジェクトがRustのDate構造体に対応します
          date: new Date().toISOString().slice(0, 10), // Date構造体のdateフィールド
          add: 0, // Date構造体のaddフィールド
          update: 0, // Date構造体のupdateフィールド
          quiz: 1, // Date構造体のquizフィールド
        },
        mode: "quiz", // これはRust関数のmode引数に対応します
      });

    } catch (err) {
      console.error("Failed to grade answer:", err);
      setError(`採点中にエラーが発生しました: ${err}`);
      toast({
        title: "採点エラー",
        description: `採点中にエラーが発生しました: ${err}`, 
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleNextQuestion = () => {
    setUserAnswer("");
    setGradingResult(null);
    if (words.length > 0) {
      const randomIndex = Math.floor(Math.random() * words.length);
      setCurrentWord(words[randomIndex]);
    }
  };

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen p-4">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle>エラー</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-red-500">{error}</p>
            {words.length === 0 && (
              <p className="mt-4">単語を追加するには、左のメニューから「単語追加」を選択してください。</p>
            )}
          </CardContent>
        </Card>
      </div>
    );
  }

  if (!currentWord) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <p>単語を読み込み中...</p>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen p-4">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle className="text-2xl font-bold text-center">
            クイズ
          </CardTitle>
          <CardDescription className="text-center">
            提示された単語の意味や使い方を記述してください。
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="text-center">
            <h3 className="text-xl font-semibold">{currentWord.vocabulary}</h3>
            <p className="text-sm text-muted-foreground">
              ({currentWord.part_of_speech.join(", ")})
            </p>
          </div>

          <Textarea
            placeholder="ここに解答を入力してください..."
            value={userAnswer}
            onChange={(e) => setUserAnswer(e.target.value)}
            rows={5}
            disabled={isLoading || gradingResult !== null}
          />

          {isLoading && (
            <div className="flex flex-col items-center space-y-2">
              <Progress value={null} className="w-full" />
              <p className="text-sm text-muted-foreground">AIが採点中...</p>
            </div>
          )}

          {gradingResult && (
            <div className="space-y-2">
              <h4 className="text-lg font-semibold">採点結果: {gradingResult.score}点</h4>
              <p className="text-sm text-muted-foreground">
                {gradingResult.feedback}
              </p>
              <div className="mt-4 p-2 border rounded-md bg-gray-50 dark:bg-gray-800">
                <h5 className="text-md font-semibold">正解例:</h5>
                <p className="text-sm">意味: {currentWord.meaning}</p>
                <p className="text-sm">翻訳: {currentWord.translate}</p>
                {currentWord.example && (
                  <p className="text-sm">例文: {currentWord.example}</p>
                )}
              </div>
            </div>
          )}
        </CardContent>
        <CardFooter className="flex justify-end space-x-2">
          {gradingResult ? (
            <Button onClick={handleNextQuestion}>次の問題へ</Button>
          ) : (
            <Button onClick={handleGradeAnswer} disabled={isLoading}>
              採点する
            </Button>
          )}
        </CardFooter>
      </Card>
    </div>
  );
}