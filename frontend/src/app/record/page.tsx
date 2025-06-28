"use client";

import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Bar, BarChart, CartesianGrid, XAxis, YAxis } from "recharts";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface DateRecord {
  date: string;
  add: number;
  update: number;
  quiz?: number;
}

const chartConfig = {
  add: {
    label: "追加",
    color: "#2563eb", // blue-600
  },
  update: {
    label: "更新",
    color: "#4ade80", // green-400
  },
  quiz: {
    label: "クイズ",
    color: "#facc15", // yellow-400
  },
} satisfies ChartConfig;

export default function RecordPage() {
  const [dateStats, setDateStats] = useState<DateRecord[]>([]);

  useEffect(() => {
    async function fetchData() {
      try {
        const dates = (await invoke("get_dates")) as DateRecord[];
        const sortedDates = [...dates].sort(
          (a, b) => new Date(a.date).getTime() - new Date(b.date).getTime()
        );
        setDateStats(sortedDates);
      } catch (error) {
        console.error("統計データの取得に失敗しました:", error);
      }
    }

    fetchData();
  }, []);

  const totals = useMemo(() => {
    return {
      add: dateStats.reduce((acc, curr) => acc + curr.add, 0),
      update: dateStats.reduce((acc, curr) => acc + curr.update, 0),
      quiz: dateStats.reduce((acc, curr) => acc + (curr.quiz || 0), 0),
    };
  }, [dateStats]);

  return (
    <div className="p-4 md:p-8">
      <Card>
        <CardHeader>
          <CardTitle>学習記録</CardTitle>
          <CardDescription>
            日々の学習活動の推移をタブで切り替えて確認できます。
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="add">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="add">追加</TabsTrigger>
              <TabsTrigger value="update">更新</TabsTrigger>
              <TabsTrigger value="quiz">クイズ</TabsTrigger>
            </TabsList>
            {(Object.keys(chartConfig) as (keyof typeof chartConfig)[]).map(
              (key) => (
                <TabsContent key={key} value={key}>
                  <div className="py-4">
                    <div className="flex items-center justify-between pb-4">
                      <div className="flex flex-col gap-1">
                        <h3 className="text-lg font-medium">
                          {chartConfig[key].label}数
                        </h3>
                        <p className="text-sm text-muted-foreground">
                          合計: {totals[key].toLocaleString()} 件
                        </p>
                      </div>
                    </div>
                    <ChartContainer
                      config={chartConfig}
                      className="aspect-auto h-[250px] w-full"
                    >
                      <BarChart
                        accessibilityLayer
                        data={dateStats}
                        margin={{
                          left: 12,
                          right: 12,
                        }}
                      >
                        <CartesianGrid vertical={false} />
                        <XAxis
                          dataKey="date"
                          tickLine={false}
                          axisLine={false}
                          tickMargin={8}
                          minTickGap={32}
                          tickFormatter={(value) => {
                            const date = new Date(value);
                            return date.toLocaleDateString("ja-JP", {
                              month: "short",
                              day: "numeric",
                            });
                          }}
                        />
                        <YAxis tickLine={false} axisLine={false} />
                        <ChartTooltip
                          cursor={false}
                          content={<ChartTooltipContent indicator="dot" />}
                        />
                        <Bar
                          dataKey={key}
                          fill={chartConfig[key].color}
                          radius={4}
                        />
                      </BarChart>
                    </ChartContainer>
                  </div>
                </TabsContent>
              )
            )}
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
}
