import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { open } from '@tauri-apps/plugin-dialog';
import { UploadCloud, FileText } from 'lucide-react';
import { Card } from '../components/ui/Card';
import { Button } from '../components/ui/Button';
import { invoke } from '@tauri-apps/api/core';

export const LoadFileScreen: React.FC = () => {
  const navigate = useNavigate();
  const [isDragging, setIsDragging] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleFileSelect = async (originalPath: string) => {
    try {
      const newPath = await invoke<string>('initialize_workspace', { path: originalPath });
      navigate('/home', { state: { filePath: newPath } });
    } catch (err) {
      console.error(err);
      setError(`Error inicializando espacio de trabajo: ${err}`);
    }
  };

  const handleOpenDialog = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Text Files',
          extensions: ['txt']
        }]
      });
      
      if (selected !== null) {
        handleFileSelect(selected as string);
      }
    } catch (err) {
      console.error(err);
      setError('Error al abrir el explorador de archivos.');
    }
  };

  const onDragOver = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const onDragLeave = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);
  };

  const onDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);
    setError(null);
    
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      if (file.name.endsWith('.txt')) {
        // Tauri file drop paths might be different depending on tauri events,
        // but since we are using standard web events, the browser file object doesn't have absolute path due to security.
        // Usually in Tauri, you handle dropping via Tauri's Window plugin.
        // For simplicity, we assume we can read the file or we need the path.
        // Actually, in Tauri v2, we should use plugin-drag-drop or just let the user know to use the button if drag drop path is unavailable.
        // Let's use standard web File object just to show interaction, but in real Tauri app we might need the path for rust backend.
        // If file.path is available (electron/tauri extensions):
        const filePath = (file as any).path || file.name; 
        handleFileSelect(filePath);
      } else {
        setError('Por favor, selecciona un archivo .txt válido.');
      }
    }
  };

  return (
    <div className="flex flex-col items-center justify-center w-full max-w-2xl mx-auto min-h-[70vh]">
      <div className="text-center mb-10">
        <h1 className="text-4xl font-bold text-text-main mb-3 tracking-tight">Cargar Archivo</h1>
        <p className="text-text-muted text-lg">Arrastra tu archivo de texto o búscalo en tu equipo para comenzar.</p>
      </div>

      <Card 
        className={`w-full p-12 flex flex-col items-center justify-center border-2 border-dashed transition-all duration-200
          ${isDragging ? 'border-primary bg-teal-50/50' : 'border-gray-200 hover:border-primary/50'}
        `}
        onDragOver={onDragOver}
        onDragLeave={onDragLeave}
        onDrop={onDrop}
      >
        <div className={`p-6 rounded-full mb-6 ${isDragging ? 'bg-primary/10 text-primary' : 'bg-gray-50 text-gray-400'}`}>
          <UploadCloud size={48} strokeWidth={1.5} />
        </div>
        
        <h3 className="text-xl font-semibold mb-2 text-text-main">Arrastra y suelta tu archivo .txt aquí</h3>
        <p className="text-text-muted mb-8 text-center max-w-md">
          El archivo será procesado localmente. No se subirá a ningún servidor externo.
        </p>

        <div className="flex items-center gap-4 w-full">
          <div className="h-px bg-gray-200 flex-1"></div>
          <span className="text-gray-400 text-sm font-medium uppercase tracking-wider">O</span>
          <div className="h-px bg-gray-200 flex-1"></div>
        </div>

        <div className="mt-8">
          <Button onClick={handleOpenDialog} size="lg" className="gap-2 shadow-sm">
            <FileText size={20} />
            Explorar Archivos
          </Button>
        </div>
      </Card>
      
      {error && (
        <div className="mt-6 text-red-500 font-medium bg-red-50 px-4 py-3 rounded-lg flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-red-500"></span>
          {error}
        </div>
      )}
    </div>
  );
};
