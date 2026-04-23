import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { open } from '@tauri-apps/plugin-dialog';
import { UploadCloud, FileText } from 'lucide-react';
import { Card } from '../components/ui/Card';
import { Button } from '../components/ui/Button';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

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

  useEffect(() => {
    let unlistenFn: (() => void) | undefined;
    
    const setupDragDrop = async () => {
      const unlisten = await getCurrentWindow().onDragDropEvent((event) => {
        if (event.payload.type === 'over') {
          setIsDragging(true);
        } else if (event.payload.type === 'drop') {
          setIsDragging(false);
          setError(null);
          const paths = (event.payload as any).paths as string[];
          if (paths && paths.length > 0) {
            const filePath = paths[0];
            if (filePath.toLowerCase().endsWith('.txt')) {
              handleFileSelect(filePath);
            } else {
              setError('Por favor, selecciona un archivo .txt válido.');
            }
          }
        } else {
          setIsDragging(false);
        }
      });
      return unlisten;
    };

    setupDragDrop().then(fn => { unlistenFn = fn; });

    return () => {
      if (unlistenFn) unlistenFn();
    };
  }, []);

  const onDragOver = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
  };

  const onDragLeave = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
  };

  const onDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
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
