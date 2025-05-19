import { z } from "zod";
export const formSchema = z.object({
  vocabulary: z.string().min(2).max(100),
  meaning: z.string().min(2).max(1000),
  translate: z.string().min(2).max(100),
  exampleSentence: z.string().min(2).max(1000),
  category: z.string(),
});

export type FormSchema = z.infer<typeof formSchema>;
