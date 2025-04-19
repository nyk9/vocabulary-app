import { z } from "zod";
export const formSchema = z.object({
  vocabulary: z.string().min(2).max(20),
  meaning: z.string().min(2).max(100),
  translate: z.string().min(2).max(100),
  exampleSentence: z.string().min(2).max(1000),
  category: z.string(),
});

export type FormSchema = z.infer<typeof formSchema>;
