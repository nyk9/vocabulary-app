import { PartOfSpeech } from "@/types/word";
import { z } from "zod";
export const formSchema = z.object({
  vocabulary: z.string().min(2).max(100),
  meaning: z.string().min(2).max(1000),
  translate: z.string().min(2).max(100),
  exampleSentence: z.string().min(2).max(1000),
  category: z.string(),
  partOfSpeech: z
    .array(z.nativeEnum(PartOfSpeech))
    .refine((value) => value.some((item) => item), {
      message: "You have to select at least one item.",
    }),
});

export type FormSchema = z.infer<typeof formSchema>;
