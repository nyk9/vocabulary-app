import AddVocabularyForm from "@/features/vocabulary/components/vocabulary-form";
import { Metadata } from "next";

export const metadata: Metadata = {
  title: "add",
};

export default function AddPage() {
  return (
    <div className="flex flex-col items-center justify-center mt-10 min-w-96 min-h-96">
      <AddVocabularyForm mode="追加" />
    </div>
  );
}
