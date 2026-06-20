import React from 'react';
import { useLocation, useNavigate, Navigate } from 'react-router-dom';
import { Activity, LayoutGrid, FileText, ArrowLeft } from 'lucide-react';
import { Card } from '../components/ui/Card';
import { EncryptDropdown } from '../components/EncryptDropdown';
import { invoke } from '@tauri-apps/api/core';

export const HomeScreen: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const filePath = location.state?.filePath;
  const originalPath = location.state?.originalPath || filePath;
  console.log("HomeScreen received filePath:", filePath);
  console.log("HomeScreen received originalPath:", originalPath);

  // Route guard.
  if (!filePath) {
    return <Navigate to="/" replace />;
  }

  const getFileName = (path: string) => {
    return path.split(/[/\\]/).pop() || path;
  };

  const handleReturn = async () => {
    try {
      await invoke('close_workspace');
    } catch (e) {
      console.error("Error closing workspace", e);
    }
    navigate('/');
  };

  return (
    <div className="flex flex-col w-full max-w-5xl mx-auto py-8">
      <div className="mb-12">
        <div className="flex items-center gap-4 mb-6">
          <button 
            onClick={handleReturn}
            className="flex items-center text-text-muted hover:text-text-main transition-colors font-medium"
          >
            <ArrowLeft size={18} className="mr-1" />
            Volver a Cargar Archivo
          </button>
          <EncryptDropdown originalFilePath={originalPath} />
        </div>
        <div className="flex items-center gap-3 text-primary mb-2">
          <FileText size={24} />
          <span className="font-semibold text-lg">{getFileName(filePath)}</span>
        </div>
        <h1 className="text-4xl font-bold text-text-main mb-3 tracking-tight">Seleccionar Algoritmo</h1>
        <p className="text-text-muted text-lg">Elige el algoritmo que deseas aplicar a tu archivo.</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        {/* Hamming Card */}
        <Card 
          className="p-10 cursor-pointer hover:border-primary hover:shadow-lg transition-all group flex flex-col items-center text-center h-[380px] justify-center"
          onClick={() => navigate('/hamming', { state: { filePath, originalPath } })}
        >
          <div className="bg-primary/10 text-primary p-6 rounded-3xl mb-8 group-hover:scale-110 transition-transform">
            <Activity size={56} strokeWidth={1.5} />
          </div>
          <h2 className="text-3xl font-semibold mb-4">Hamming</h2>
          <p className="text-text-muted text-lg px-4">Protección y detección de errores usando código Hamming.</p>
        </Card>

        {/* Huffman Card */}
        <Card 
          className="p-10 cursor-pointer hover:border-blue-500 hover:shadow-lg transition-all group flex flex-col items-center text-center h-[380px] justify-center"
          onClick={() => navigate('/huffman', { state: { filePath, originalPath } })}
        >
          <div className="bg-blue-50 text-blue-500 p-6 rounded-3xl mb-8 group-hover:scale-110 transition-transform">
            <LayoutGrid size={56} strokeWidth={1.5} />
          </div>
          <h2 className="text-3xl font-semibold mb-4">Huffman</h2>
          <p className="text-text-muted text-lg px-4">Compresión de archivos sin pérdida de información.</p>
        </Card>
      </div>
    </div>
  );
};
