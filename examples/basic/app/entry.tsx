import './globals.css';
import { createRoot } from 'react-dom/client';
import HomePage from './page';

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(<HomePage />);
}
