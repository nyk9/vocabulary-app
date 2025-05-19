import AddVocabularyForm from "@/features/vocabulary/components/vocabulary-form";

export async function generateStaticParams() {
  const ids: string[] = Array.from({ length: 10000 }, (_, i) => String(i));
  return ids.map((i) => ({
    id: i,
  }));
}

export default async function UpdatePage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  return (
    <div className="flex flex-col items-center justify-center mt-10 min-w-96 min-h-96">
      <AddVocabularyForm mode="更新" wordId={Number(id)} />
    </div>
  );
}
