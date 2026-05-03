import React, { useState, useEffect } from 'react';
import { useLocation, useNavigate, Navigate } from 'react-router-dom';
import { ArrowLeft, LayoutGrid, Unlock, FileCode, CheckCircle, FileText, AlertCircle, FileArchive } from 'lucide-react';
import { Card } from '../components/ui/Card';
import { Button } from '../components/ui/Button';
import { invoke } from '@tauri-apps/api/core';

export const HuffmanScreen: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const filePath = location.state?.filePath;

  const [compactMode, setCompactMode] = useState<'words' | 'chars'>('words');
  const [generatedFiles, setGeneratedFiles] = useState<{name: string, size: number}[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [toast, setToast] = useState<{ message: string, type: 'success' | 'error' | 'warning' } | null>(null);
  const [originalSize, setOriginalSize] = useState<number | null>(null);

  // Show toast notification in UI
  const showToast = (message: string, type: 'success' | 'error' | 'warning' = 'success') => {
    setToast({ message, type });
    setTimeout(() => {
      setToast(null);
    }, 2000);
  };

  // Helper to format bytes to human-readable size
  const formatBytes = (bytes: number, decimals = 2) => {
    if (!+bytes) return '0 Bytes'
    const k = 1024
    const dm = decimals < 0 ? 0 : decimals
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
  }

  // Initial load and refresh of files from the workspace
  const refreshWorkspace = async () => {
    try {
      const files = await invoke<string[]>('list_workspace_files'); //invoke rust function (back-end)
     
      // Filter the base file (the original) so it doesn't appear in "Generated"
      const baseName = filePath?.split(/[/\\]/).pop();
      const generated = files.filter(f => f !== baseName);
      
      const parentDir = filePath.substring(0, Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\')));
      const separator = filePath.includes('\\') ? '\\' : '/';

      const filesWithSize = await Promise.all(generated.map(async (fileName) => {
        const fullPath = `${parentDir}${separator}${fileName}`;
        try {
          const size = await invoke<number>('get_file_size', { path: fullPath });
          return { name: fileName, size };
        } catch (e) {
          console.error(`Error getting size for ${fileName}:`, e);
          return { name: fileName, size: 0 };
        }
      }));

      setGeneratedFiles(filesWithSize);
      return filesWithSize;
    } catch (err) {
      console.error("Error listing workspace files:", err);
      return [];
    }
  };

  useEffect(() => {
    refreshWorkspace();
    if (filePath) {
      invoke<number>('get_file_size', { path: filePath })
        .then(size => setOriginalSize(size))
        .catch(e => console.error("Error getting original file size", e));
    }
  }, [filePath]);

  // Route guard.
  if (!filePath) {
    return <Navigate to="/" replace />;
  }

  const getFileName = (path: string) => {
    return path.split(/[/\\]/).pop() || path;
  };

  const handleCompact = async () => {
    try {
      await invoke<string>('compress_huffman', { 
        path: filePath, 
        modeStr: compactMode
      });
      
      const newFiles = await refreshWorkspace();
      if (newFiles.length > 0) {
        // Find the newly generated .huf file and select it
        const hufFiles = newFiles.filter(f => f.name.endsWith('.huf'));
        if (hufFiles.length > 0) {
          setSelectedFile(hufFiles[hufFiles.length - 1].name);
        }
      }
      showToast("Archivo compactado con éxito.", "success");
    } catch (error) {
      console.error("Error during compression:", error);
      showToast(`Error al compactar el archivo: ${error}`, 'error');
    }
  };

  const handleExtract = async () => {
    if (!selectedFile) return;
    try {
      const parentDir = filePath.substring(0, Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\')));
      const separator = filePath.includes('\\') ? '\\' : '/';
      const fullPathToSelected = `${parentDir}${separator}${selectedFile}`;

      await invoke<string>('extract_huffman', { path: fullPathToSelected, modeStr: compactMode });
      
      showToast(`Archivo descompactado con éxito.`, 'success');
      
      const newFiles = await refreshWorkspace();
      if (newFiles.length > 0) {
        // Select the newly extracted file
        const extractedFiles = newFiles.filter(f => f.name.includes('extracted'));
        if (extractedFiles.length > 0) {
          setSelectedFile(extractedFiles[extractedFiles.length - 1].name);
        }
      }
    } catch (error) {
      console.error("Error during extraction:", error);
      showToast(`Error al descompactar el archivo: ${error}`, 'warning');
    }
  };

  return (
    <div className="flex flex-col w-full max-w-6xl mx-auto py-6 h-full relative">
      {/* Toast Notification */}
      {toast && (
        <div className="fixed top-6 left-1/2 transform -translate-x-1/2 z-50 animate-in fade-in slide-in-from-top-4 duration-300">
          <div className={`px-6 py-4 rounded-xl shadow-lg flex items-center gap-3 ${
            toast.type === 'success' 
              ? 'bg-white border-l-4 border-green-500 text-green-800' 
              : toast.type === 'warning'
              ? 'bg-white border-l-4 border-yellow-500 text-yellow-800'
              : 'bg-white border-l-4 border-red-500 text-red-800'
          }`}>
            {toast.type === 'success' ? <CheckCircle className="text-green-500" size={24} /> : 
             toast.type === 'warning' ? <AlertCircle className="text-yellow-500" size={24} /> :
             <AlertCircle className="text-red-500" size={24} />}
            <p className="font-medium whitespace-pre-wrap">{toast.message}</p>
          </div>
        </div>
      )}

      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div>
          <button 
            onClick={() => navigate('/home', { state: { filePath } })}
            className="flex items-center text-text-muted hover:text-text-main transition-colors mb-2 font-medium"
          >
            <ArrowLeft size={18} className="mr-1" />
            Volver a Selección
          </button>
          <div className="flex items-center gap-2">
            <h1 className="text-3xl font-bold text-text-main tracking-tight">Huffman</h1>
            <span className="px-3 py-1 bg-blue-50 text-blue-500 text-sm font-semibold rounded-full flex items-center gap-2">
              {getFileName(filePath)}
              {originalSize !== null && (
                <span className="opacity-70 font-normal">({formatBytes(originalSize)})</span>
              )}
            </span>
          </div>
        </div>
        
        <Button 
          variant="outline" 
          className="gap-2"
          onClick={() => navigate('/compare', { state: { filePath, sourceScreen: '/huffman' } })}
          disabled={generatedFiles.length === 0}
        >
          <FileText size={18} />
          Comparar Archivos
        </Button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
        {/* Left Column - Configuration */}
        <Card className="p-6 lg:col-span-5">
          <div className="flex items-center gap-2 mb-6 border-b pb-4">
            <LayoutGrid className="text-blue-500" size={24} />
            <h2 className="text-xl font-semibold">Configuración de Compresión</h2>
          </div>

          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-3">Modo de Compresión</label>
              <div className="flex gap-4">
                {['words', 'chars'].map((mode) => (
                  <label key={mode} className={`flex-1 cursor-pointer rounded-xl border-2 p-4 flex flex-col items-center justify-center transition-all ${compactMode === mode ? 'border-blue-500 bg-blue-50 text-blue-600 font-semibold' : 'border-gray-200 text-gray-500 hover:border-gray-300'}`}>
                    <input type="radio" name="compactMode" className="hidden" checked={compactMode === mode} onChange={() => setCompactMode(mode as 'words' | 'chars')} />
                    <span className="flex items-center gap-2">{compactMode === mode && <CheckCircle size={18} />} {mode === 'words' ? 'Por Palabras' : 'Por Caracteres'}</span>
                  </label>
                ))}
              </div>
            </div>

            <div className="pt-4 mt-4 border-t border-gray-100">
              <Button onClick={handleCompact} fullWidth size="lg" className="gap-2 text-lg shadow-md bg-blue-600 hover:bg-blue-700">
                <FileArchive size={22} />
                COMPACTAR
              </Button>
            </div>
          </div>
        </Card>

        {/* Right Column - Generated Files */}
        <Card className="p-6 lg:col-span-7 h-full flex flex-col">
          <div className="flex items-center gap-2 mb-6 border-b pb-4">
            <FileCode className="text-blue-500" size={24} />
            <h2 className="text-xl font-semibold">Archivos Generados</h2>
          </div>

          {generatedFiles.length === 0 ? (
            <div className="flex-1 flex flex-col items-center justify-center text-gray-400 py-12">
              <FileCode size={48} className="mb-4 opacity-20" />
              <p>Configura y compacta tu archivo para ver los resultados aquí.</p>
            </div>
          ) : (
            <div className="flex-1 flex flex-col">
              <div className="space-y-3 mb-6 flex-1">
                {generatedFiles.map((fileObj, idx) => {
                  const file = fileObj.name;
                  const isExtracted = file.includes('extracted');
                  return (
                  <div 
                    key={idx}
                    onClick={() => setSelectedFile(file)}
                    className={`p-4 rounded-xl border-2 cursor-pointer transition-all flex items-center justify-between
                      ${isExtracted 
                        ? (selectedFile === file ? 'border-gray-300 bg-gray-100' : 'border-gray-100 bg-gray-50 opacity-70 hover:border-gray-200')
                        : (selectedFile === file ? 'border-blue-500 bg-blue-50' : 'border-gray-100 hover:border-gray-200')}
                    `}
                  >
                    <div className="flex items-center gap-3">
                      <div className={`p-2 rounded-lg ${isExtracted ? 'bg-gray-200 text-gray-500' : selectedFile === file ? 'bg-blue-100 text-blue-600' : 'bg-gray-100 text-gray-500'}`}>
                        <FileText size={20} />
                      </div>
                      <div className="flex flex-col">
                        <span className={`font-medium ${isExtracted ? 'text-gray-500' : selectedFile === file ? 'text-blue-700' : 'text-text-main'}`}>
                          {file}
                        </span>
                        <span className="text-xs text-gray-500">
                          {formatBytes(fileObj.size)}
                        </span>
                      </div>
                    </div>
                    {selectedFile === file && <CheckCircle className={isExtracted ? "text-gray-500" : "text-blue-500"} size={20} />}
                  </div>
                )})}
              </div>

              <div className="pt-4 border-t border-gray-100 mt-auto">
                <Button 
                  onClick={handleExtract} 
                  variant="secondary" 
                  fullWidth 
                  size="lg" 
                  className={`gap-2 text-lg shadow-md ${selectedFile && selectedFile.includes('extracted') ? 'grayscale' : ''}`}
                  disabled={!selectedFile || selectedFile.includes('extracted')}
                >
                  <Unlock size={22} />
                  DESCOMPACTAR
                </Button>
                <p className="text-xs text-center text-gray-400 mt-3">
                  La descompactación se aplicará sobre el archivo seleccionado en la lista.
                </p>
              </div>
            </div>
          )}
        </Card>
      </div>
    </div>
  );
};
