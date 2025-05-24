"use client";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { formSchema } from "../lib/formSchema";
import { Textarea } from "@/components/ui/textarea";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "@/hooks/use-toast";
import { useEffect } from "react";
import { Word } from "@/types/word";

type VocabularyFormProps = {
  mode: "追加" | "更新";
  wordId?: number;
};

export default function AddVocabularyForm({
  mode = "追加",
  wordId,
}: VocabularyFormProps) {
  const { toast } = useToast();
  const form = useForm({
    resolver: zodResolver(formSchema),
    defaultValues: {
      vocabulary: "",
      meaning: "",
      translate: "",
      exampleSentence: "",
      category: "",
    },
  });

  useEffect(() => {
    if (mode === "更新" && wordId) {
      // 更新モードの場合、指定したidの単語情報を取得し、フォームの初期値にする
      const fetchWord = async () => {
        try {
          const word: Word = await invoke("get_words_by_id", { id: wordId });
          form.reset({
            vocabulary: word.vocabulary,
            meaning: word.meaning,
            translate: word.translate,
            exampleSentence: word.example || "",
            category: word.category,
          });
        } catch (error) {
          console.error("", error);
        }
      };
      fetchWord();
    }
  }, [mode, wordId]);

  async function onSubmit(value: z.infer<typeof formSchema>) {
    try {
      const { vocabulary, meaning, translate, exampleSentence, category } =
        value;

      if (mode == "追加") {
        // Rustの単語追加関数を呼び込み、実行する
        await invoke<string>("add_word", {
          vocabulary,
          meaning,
          translate,
          example: exampleSentence,
          category,
        });
        console.log("add_word が成功しました");
      } else if (mode == "更新" && wordId) {
        // Rustの単語更新関数を呼び込み、実行する
        await invoke<string>("update_word", {
          id: wordId,
          vocabulary,
          meaning,
          translate,
          example: exampleSentence,
          category,
        });
      }

      // フォームの内容を""に初期化する
      form.reset({
        vocabulary: "",
        meaning: "",
        translate: "",
        exampleSentence: "",
        category: "",
      });

      toast({
        title: `単語を${mode}しました`,
        description: `『${vocabulary}』を${mode}しました。`,
      });
    } catch (error) {
      // エラーの詳細を表示
      console.error(`単語の${mode}に失敗しました:`, error);
      toast({
        title: "エラー",
        description: `単語の${mode}に失敗しました: ${error}`,
        variant: "destructive",
      });
    }
  }

  return (
    <Card className="flex flex-col space-x-2">
      <CardHeader>
        <CardTitle>単語{mode}</CardTitle>
        <CardDescription>単語を{mode}します</CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-3 w-1/2 px-7"
          >
            <FormField
              control={form.control}
              name="vocabulary"
              render={({ field }) => (
                <FormItem className="space-y-6 p-1">
                  <FormLabel>単語</FormLabel>
                  <FormControl>
                    <Input placeholder="単語" {...field} />
                  </FormControl>

                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="meaning"
              render={({ field }) => (
                <FormItem className="space-y-6 p-1">
                  <FormLabel>意味</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder="意味"
                      className="resize-none"
                      {...field}
                    />
                  </FormControl>

                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="translate"
              render={({ field }) => (
                <FormItem className="space-y-6 p-1">
                  <FormLabel>翻訳</FormLabel>
                  <FormControl>
                    <Input placeholder="翻訳" {...field} />
                  </FormControl>

                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="exampleSentence"
              render={({ field }) => (
                <FormItem className="space-y-6 p-1">
                  <FormLabel>例文</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder="例文"
                      className="resize-none"
                      {...field}
                    />
                  </FormControl>

                  <FormMessage />
                </FormItem>
              )}
            />
            {/* <FormField
              control={form.control}
              name="category"
              render={({ field }) => (
                <FormItem className="space-y-6 p-1">
                  <FormLabel>カテゴリー</FormLabel>
                  <FormControl></FormControl>
                  <FormMessage />
                </FormItem>
              )}
            /> */}
            <FormField
              control={form.control}
              name="category"
              render={({ field }) => (
                <FormItem className="space-y-6 p-1">
                  <FormLabel>カテゴリー</FormLabel>
                  <FormControl>
                    <Input type="text" placeholder="カテゴリー" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit" className="space-y-6 m-1">
              Submit
            </Button>
          </form>
        </Form>
      </CardContent>
    </Card>
  );
}
