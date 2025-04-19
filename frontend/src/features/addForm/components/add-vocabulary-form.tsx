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

export default function AddVocabularyForm() {
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

  async function onSubmit(value: z.infer<typeof formSchema>) {
    try {
      const { vocabulary, meaning, translate, exampleSentence, category } =
        value;
      const date = new Date();
      await invoke<string>("add_word", {
        vocabulary,
        meaning,
        translate,
        example: exampleSentence,
        category,
      });
      form.reset({
        vocabulary: "",
        meaning: "",
        translate: "",
        exampleSentence: "",
        category: "",
      });

      toast({
        title: "単語を追加しました",
        description: `『${vocabulary}』を追加しました。`,
      });
    } catch (error) {
      console.error("単語の追加に失敗しました", error);
      toast({
        title: "エラー",
        description: "単語の追加に失敗しました",
      });
    }
  }

  return (
    <Card className="flex flex-col space-x-2">
      <CardHeader>
        <CardTitle>単語追加</CardTitle>
        <CardDescription>単語を追加します</CardDescription>
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
                    <Input placeholder="意味" {...field} />
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
