import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [focusedAppName, setFocusedAppName] = useState("알 수 없음");
  const [timer, setTimer] = useState(0);
  const alarmAudio = useRef<HTMLAudioElement | null>(null);

  // 타이머 시간을 HH:MM:SS 형식으로 포맷팅하는 함수
  function formatTime(seconds: number): string {
    const hrs = Math.floor(seconds / 3600)
      .toString()
      .padStart(2, "0");
    const mins = Math.floor((seconds % 3600) / 60)
      .toString()
      .padStart(2, "0");
    const secs = (seconds % 60).toString().padStart(2, "0");
    return `${hrs}:${mins}:${secs}`;
  }

  useEffect(() => {
    // 창 크기 조정
    async function adjustWindowSize() {
      try {
        const width = window.innerWidth;
        const height = window.innerHeight;
        await invoke("set_window_size", { width, height });
      } catch (error) {
        console.error("Failed to set window size:", error);
      }
    }

    adjustWindowSize();

    let intervalId;
    let timerId;

    async function updateFocusedAppName() {
      try {
        const appName: string = await invoke("get_focused_app_name");
        setFocusedAppName(appName);
      } catch (error) {
        console.error(error);
        setFocusedAppName("Error retrieving app name");
      }
    }

    // 포커싱된 앱 업데이트 주기
    intervalId = setInterval(updateFocusedAppName, 1000);

    // 타이머 증가 주기
    timerId = setInterval(() => {
      if (focusedAppName === "Clip Studio" || focusedAppName.toLowerCase().includes("paint")) {
        setTimer((prevTimer) => prevTimer + 1);

        // 10초마다 알람 소리 재생 (개발 환경에서는 10초, 실제로는 600초)
        if ((timer + 1) % 600 === 0 && alarmAudio.current) {
          alarmAudio.current.play();
        }
      }
    }, 1000);

    return () => {
      clearInterval(intervalId);
      clearInterval(timerId);
    };
  }, [focusedAppName, timer]);

  return (
    <div className="container">
      <h3>{focusedAppName}</h3>
      <h2>{formatTime(timer)}</h2>
      {/* 알람 오디오 요소 추가 */}
      <audio ref={alarmAudio} src="alarm.mp3" preload="auto" />
    </div>
  );
}

export default App;
