import '../globals.css';

interface ButtonProps {
  children?: React.ReactNode;
  variant?: 'primary' | 'secondary';
  onClick?: () => void;
}

export function Button({ children, variant = 'primary', onClick }: ButtonProps) {
  const baseStyles = 'inline-flex items-center justify-center font-medium rounded-lg transition-colors px-6 py-3';

  const variants = {
    primary: 'bg-blue-600 text-white hover:bg-blue-700',
    secondary: 'bg-gray-600 text-white hover:bg-gray-700',
  };

  return (
    <button type="button" onClick={onClick} className={`${baseStyles} ${variants[variant]}`}>
      {children || 'Remote Button'}
    </button>
  );
}

export default Button;
