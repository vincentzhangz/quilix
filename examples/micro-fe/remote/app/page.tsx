import { Button } from "./components/Button";
import "./globals.css";

export default function HomePage() {
  return (
    <main className="min-h-screen bg-gray-800 flex flex-col items-center justify-center p-8">
      <h1 className="text-3xl font-bold text-white mb-4">
        Quilix Micro-FE Remote
      </h1>
      <p className="text-gray-400 mb-8">
        This component is exposed via Module Federation
      </p>
      <Button />
    </main>
  );
}
