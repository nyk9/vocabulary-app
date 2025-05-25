import { Word } from "@/types/word";
import Anthropic from "@anthropic-ai/sdk";
import { NextRequest, NextResponse } from "next/server";

const anthropic = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY,
});

export async function POST(req: NextRequest) {
  try {
    const { vocabulary } = await req.json();
    const wordCount: number = vocabulary.length;
    // vocabulary: Word[]の中から categoryだけを取り出した配列
    const categories: string[] = vocabulary.map((voc: Word) => voc.category);
    const existingWords: Word[] = vocabulary;

    const prompt = `<task>
    Analyse the user's vocabulary and recommend 5 new English words for a beginner Japanese learner.
    </task>

    <current_status>
      <total_words> ${wordCount} </total_words>
      <category_distribution> ${JSON.stringify(categories)} </category_distribution
    <current_status>

    <existing_words>
      ${existingWords
        .map(
          (word) => `
        vocabulary: ${word.vocabulary},
        meaning: ${word.meaning},
        translate: ${word.translate},
        example: ${word.example || "there is no example"}
        ${word.category || "uncategorized"}`,
        )
        .join("\n")}
    </existing_words>

    <criteria>
    1. No duplicates from existing words
    2. Appropriate difficulty (slightly challenging is good)
    3. High practical value for daily/business use
    4. Related to existing words for systematic vocabulary building
    5. Memorable and distinctive
    </criteria>

    <output_format>
    Respond in this exact JSON format:

    \`\`\`json
      {
        "recommendations": [
          {
            "vocabulary": "word",
            "meaning": "why recommended (connection to existing words)",
            "translate": "日本語意味",
            "category": "category_name",
            "example": "Example sentence in English",
          }
        ],
        "learningAdvice": "Overall learning advice in Japanese"
      }
      \`\`\`
    </output_format>

    <instruction>
      Focus on words that build upon existing vocabulary and maintain learning motivation.
    </instruction>
  `;

    const msg = await anthropic.messages.create({
      model: "claude-3-5-haiku-20241022",
      max_tokens: 512,
      messages: [
        {
          role: "user",
          content: prompt,
        },
      ],
    });

    return NextResponse.json(msg);
  } catch (error) {
    console.error("Error processing suggestion request:", error);
    return NextResponse.json(
      { error: `Failed to process request: ${error}` },
      { status: 500 },
    );
  }
}
