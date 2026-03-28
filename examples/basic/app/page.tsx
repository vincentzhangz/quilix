import { Button } from "./components/Button";

export default function HomePage() {
  return (
    <main className="min-h-screen bg-linear-to-br from-gray-900 to-gray-800 flex flex-col items-center justify-center p-8">
      <div className="max-w-2xl w-full space-y-8 text-center">
        <h1 className="text-5xl font-bold text-white tracking-tight">
          Welcome to <span className="text-blue-500">Quilix</span>
        </h1>

        <p className="text-xl text-gray-300">
          A React framework powered by Rust and Rspack
        </p>

        <div className="flex flex-wrap gap-4 justify-center pt-4">
          <Button variant="primary" size="lg">
            Get Started
          </Button>
          <Button variant="outline" size="lg">
            Learn More
          </Button>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-8">
          <FeatureCard
            title="Fast"
            description="Powered by Rust for blazing fast performance"
          />
          <FeatureCard
            title="Simple"
            description="File-based routing with React conventions"
          />
          <FeatureCard
            title="Flexible"
            description="Module Federation for micro-frontends"
          />
        </div>
      </div>
    </main>
  );
}

function FeatureCard({
  title,
  description,
}: {
  title: string;
  description: string;
}) {
  return (
    <div className="bg-gray-800/50 backdrop-blur border border-gray-700 rounded-xl p-6 text-left">
      <h3 className="text-lg font-semibold text-white mb-2">{title}</h3>
      <p className="text-gray-400 text-sm">{description}</p>
    </div>
  );
}
