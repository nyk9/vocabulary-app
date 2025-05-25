"use client";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";

interface DateRecord {
  date: string;
  add: number;
  update: number;
  quiz?: number;
}

export default function WordStats() {
  const [dateStats, setDateStats] = useState<DateRecord[]>([]);

  useEffect(() => {
    async function fetchData() {
      try {
        const dates = (await invoke("get_dates")) as DateRecord[];
        // 日付順にソートする
        const sortedDates = [...dates].sort(
          (a, b) => new Date(a.date).getTime() - new Date(b.date).getTime(),
        );
        setDateStats(sortedDates);
      } catch (error) {
        console.error("統計データの取得に失敗しました:", error);
      }
    }

    fetchData();
  }, []);

  return (
    <div className="chart-container" style={{ width: "100%", height: 400 }}>
      <h2>単語追加の統計</h2>
      <ResponsiveContainer width="100%" height="100%">
        <BarChart
          data={dateStats}
          margin={{ top: 20, right: 30, left: 20, bottom: 5 }}
        >
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="date" />
          <YAxis />
          <Tooltip />
          <Legend />
          <Bar dataKey="add" name="追加した単語数" fill="#8884d8" />
          <Bar dataKey="update" name="更新した単語数" fill="#82ca9d" />
          {/* クイズ情報があれば表示 */}
          {dateStats.some((stat) => stat.quiz !== undefined) && (
            <Bar dataKey="quiz" name="クイズ実施回数" fill="#ffc658" />
          )}
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
