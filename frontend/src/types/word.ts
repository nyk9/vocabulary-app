export interface Word {
  id: number;
  vocabulary: string;
  meaning: string;
  translate: string;
  category: string;
  partOfSpeech: PartOfSpeech[];
  example?: string;
}

export enum PartOfSpeech {
  Noun = "Noun",
  Verb = "Verb",
  Adjective = "Adjective",
  Adverb = "Adverb",
  Pronoun = "Pronoun",
  AuxiliaryVerb = "AuxiliaryVerb",
  Article = "Article",
  Conjunction = "Conjunction",
  Preposition = "Preposition",
  Interjection = "Interjection",
  Other = "Other",
}
