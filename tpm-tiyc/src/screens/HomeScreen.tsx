import React from 'react';
import { useLocation, useNavigate, Navigate } from 'react-router-dom';
import { Activity, LayoutGrid, HelpCircle, FileText, ArrowLeft } from 'lucide-react';
import { Card } from '../components/ui/Card';

export const HomeScreen: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const filePath = location.state?.filePath;

  if (!filePath) {
    // Navigate back to load file if there's no file in context
    return <Navigate to="/" replace />;
  }

  const getFileName = (path: string) => {
    return path.split(/[/\\]/).pop() || path;
  };

  return (
    <div className="flex flex-col w-full max-w-5xl mx-auto py-8">
      <div className="mb-12">
        <button 
          onClick={() => navigate('/')}
          className="flex items-center text-text-muted hover:text-text-main transition-colors mb-6 font-medium"
        >
          <ArrowLeft size={18} className="mr-1" />
          Volver a Cargar Archivo
        </button>
        <div className="flex items-center gap-3 text-primary mb-2">
          <FileText size={24} />
          <span className="font-semibold text-lg">{getFileName(filePath)}</span>
        </div>
        <h1 className="text-4xl font-bold text-text-main mb-3 tracking-tight">Seleccionar Algoritmo</h1>
        <p className="text-text-muted text-lg">Elige el algoritmo que deseas aplicar a tu archivo.</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* Hamming Card */}
        <Card 
          className="p-8 cursor-pointer hover:border-primary hover:shadow-md transition-all group flex flex-col items-center text-center h-72 justify-center"
          onClick={() => navigate('/hamming', { state: { filePath } })}
        >
          <div className="bg-primary/10 text-primary p-5 rounded-2xl mb-6 group-hover:scale-110 transition-transform">
            <Activity size={40} strokeWidth={1.5} />
          </div>
          <h2 className="text-2xl font-semibold mb-2">Hamming</h2>
          <p className="text-text-muted">Protección y detección de errores usando código Hamming.</p>
        </Card>

        {/* Huffman Card (Not implemented) */}
        <Card 
          className="p-8 cursor-not-allowed opacity-60 flex flex-col items-center text-center h-72 justify-center bg-gray-50 border-transparent"
        >
          <div className="bg-gray-200 text-gray-500 p-5 rounded-2xl mb-6">
            <HelpCircle size={40} strokeWidth={1.5} />
          </div>
          <h2 className="text-2xl font-semibold mb-2">Próximamente</h2>
          <p className="text-text-muted">Más algoritmos estarán disponibles en el futuro.</p>
        </Card>

        {/* Coming Soon Card */}
        <Card 
          className="p-8 cursor-not-allowed opacity-60 flex flex-col items-center text-center h-72 justify-center bg-gray-50 border-transparent"
        >
          <div className="bg-gray-200 text-gray-500 p-5 rounded-2xl mb-6">
            <HelpCircle size={40} strokeWidth={1.5} />
          </div>
          <h2 className="text-2xl font-semibold mb-2">Próximamente</h2>
          <p className="text-text-muted">Más algoritmos estarán disponibles en el futuro.</p>
        </Card>
      </div>
    </div>
  );
};
