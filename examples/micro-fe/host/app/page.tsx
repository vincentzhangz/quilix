import type React from "react";
import { useEffect, useState } from "react";

export default function HomePage() {
  const [RemoteButton, setRemoteButton] =
    useState<React.ComponentType<any> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Dynamically import the Button from the remote
    // @ts-expect-error - Module Federation dynamic import
    import("remote/Button")
      .then((mod) => {
        setRemoteButton(() => mod.default);
        setLoading(false);
      })
      .catch((err) => {
        console.error("Failed to load remote Button:", err);
        setError(
          "Failed to load remote Button. Make sure remote is running on port 3001",
        );
        setLoading(false);
      });
  }, []);

  return (
    <main className="min-h-screen bg-slate-900 flex flex-col items-center justify-center p-8">
      <h1 className="text-4xl font-bold text-white mb-4">
        Quilix Micro-FE Host
      </h1>
      <p className="text-gray-400 mb-8">Module Federation Demo</p>
      <div id="button-container" className="text-white">
        {loading && <p>Loading remote Button...</p>}
        {error && <p className="text-red-500">{error}</p>}
        {RemoteButton && <RemoteButton />}
      </div>
    </main>
  );
}
