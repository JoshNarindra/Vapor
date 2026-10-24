import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";
import type { ReactNode } from "react";
import "./App.css";

type SystemMemoryReading = {
  totalBytes: number;
  availableBytes: number;
  usedBytes: number;
};

type MemoryState = "loading" | "ready" | "error";

function errorMessage(error: unknown): string {
  if (typeof error === "string") return error;
  if (error instanceof Error) return error.message;
  if (typeof error === "object" && error !== null && "message" in error && typeof error.message === "string") {
    return error.message;
  }
  return "Unable to read system memory.";
}

type MetricCardProps = {
  title: string;
  eyebrow: string;
  className: string;
  state?: string;
  children: ReactNode;
};

function MetricCard({ title, eyebrow, className, state = "No sample", children }: MetricCardProps) {
  return (
    <article className={`metric-card ${className}`}>
      <div className="metric-card-header">
        <div>
          <p className="metric-eyebrow">{eyebrow}</p>
          <h2>{title}</h2>
        </div>
        <span className="metric-state">{state}</span>
      </div>
      {children}
    </article>
  );
}

function App() {
  const [memory, setMemory] = useState<SystemMemoryReading | null>(null);
  const [memoryState, setMemoryState] = useState<MemoryState>("loading");
  const [memoryError, setMemoryError] = useState<string | null>(null);
  const [sampledAt, setSampledAt] = useState<Date | null>(null);

  const refreshMemory = useCallback(async () => {
    setMemoryState("loading");
    setMemoryError(null);

    try {
      const reading = await invoke<SystemMemoryReading>("read_system_memory");
      setMemory(reading);
      setSampledAt(new Date());
      setMemoryState("ready");
    } catch (error) {
      setMemory(null);
      setMemoryError(errorMessage(error));
      setMemoryState("error");
    }
  }, []);

  useEffect(() => {
    void refreshMemory();
  }, [refreshMemory]);

  const memoryStateLabel = {
    loading: "Reading…",
    ready: "Sampled",
    error: "Unavailable",
  }[memoryState];
  const gibibytes = (bytes: number) => (bytes / 1024 ** 3).toFixed(2);

  return (
    <div className="app-shell">
      <header className="topbar">
        <a className="brand" href="#overview" aria-label="SteamOS Hub overview">
          <span className="brand-mark" aria-hidden="true">SH</span>
          <span>SteamOS Hub</span>
        </a>
        <nav className="main-nav" aria-label="Main navigation">
          <a href="#overview" aria-current="page">System</a>
        </nav>
        <span className="topbar-context">DESKTOP MODE</span>
      </header>

      <main id="overview">
        <section className="page-heading" aria-labelledby="page-title">
          <div>
            <p className="eyebrow">TASK MANAGER <span aria-hidden="true">/</span> OVERVIEW</p>
            <h1 id="page-title">System overview</h1>
            <p className="page-subtitle">A clear view of the resources in use across your machine.</p>
          </div>
          <span className="read-only-badge"><span aria-hidden="true" />READ ONLY</span>
        </section>

        <aside className="connection-notice" role="status">
          <span className="notice-mark" aria-hidden="true">i</span>
          <p><strong>Memory is connected; other readings are not yet available.</strong> RAM comes from Linux system memory information. CPU, GPU, disk and network remain blank until their readers are implemented.</p>
        </aside>

        <section className="metric-grid" aria-label="Whole-machine resource readings">
          <MetricCard title="CPU" eyebrow="PROCESSOR" className="cpu-card">
            <div className="main-reading"><span>—</span><span className="unit">%</span></div>
            <p className="reading-caption">of total CPU capacity</p>
            <div className="card-footnote">Whole-machine utilization</div>
          </MetricCard>

          <MetricCard title="Memory" eyebrow="PHYSICAL RAM" className="memory-card" state={memoryStateLabel}>
            <div className="main-reading memory-reading"><span>{memory ? gibibytes(memory.usedBytes) : "—"} <span className="unit">/</span> {memory ? gibibytes(memory.totalBytes) : "—"}</span><span className="unit">GiB</span></div>
            <p className="reading-caption">used <span aria-hidden="true">/</span> total</p>
            <div className="card-divider"><span>Available</span><strong>{memory ? `${gibibytes(memory.availableBytes)} GiB` : "— GiB"}</strong></div>
            <button className="refresh-button" type="button" onClick={() => void refreshMemory()} disabled={memoryState === "loading"}>
              {memoryState === "loading" ? "Reading memory…" : "Refresh memory"}
            </button>
            {memoryError && <p className="memory-error">{memoryError}</p>}
          </MetricCard>

          <MetricCard title="GPU" eyebrow="GRAPHICS" className="gpu-card">
            <div className="main-reading"><span>—</span><span className="unit">%</span></div>
            <p className="reading-caption">overall GPU activity</p>
            <div className="card-divider"><span>GPU memory</span><strong>— / — GiB</strong></div>
          </MetricCard>

          <MetricCard title="Disk activity" eyebrow="STORAGE" className="disk-card">
            <div className="rate-grid">
              <div className="rate-reading"><span className="rate-arrow receive" aria-hidden="true">↓</span><span>—</span><span className="unit">MiB/s</span></div>
              <div className="rate-reading"><span className="rate-arrow send" aria-hidden="true">↑</span><span>—</span><span className="unit">MiB/s</span></div>
              <span className="rate-label">Read</span>
              <span className="rate-label">Write</span>
            </div>
            <div className="card-footnote">Combined system-wide activity</div>
          </MetricCard>

          <MetricCard title="Network" eyebrow="CONNECTIVITY" className="network-card">
            <div className="rate-grid">
              <div className="rate-reading"><span className="rate-arrow receive" aria-hidden="true">↓</span><span>—</span><span className="unit">Mbit/s</span></div>
              <div className="rate-reading"><span className="rate-arrow send" aria-hidden="true">↑</span><span>—</span><span className="unit">Mbit/s</span></div>
              <span className="rate-label">Received</span>
              <span className="rate-label">Sent</span>
            </div>
            <div className="card-footnote">Combined system-wide activity</div>
          </MetricCard>
        </section>

        <footer className="page-footer">
          <span className="footer-indicator" aria-hidden="true" />
          <span>{sampledAt ? `RAM sampled ${sampledAt.toLocaleTimeString()}` : memoryState === "loading" ? "Reading RAM" : "RAM sample unavailable"}</span>
          <span className="footer-separator" aria-hidden="true">·</span>
          <span>Used RAM = total minus Linux MemAvailable. Other metrics are not connected.</span>
        </footer>
      </main>
    </div>
  );
}

export default App;
