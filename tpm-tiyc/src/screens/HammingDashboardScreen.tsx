
import React, { useState } from 'react';
import { useLocation, useNavigate, Navigate } from 'react-router-dom';
import { ArrowLeft, Shield, Unlock, FileCode, CheckCircle, FileText } from 'lucide-react';
import { Card } from '../components/ui/Card';
import { Button } from '../components/ui/Button';
import { invoke } from '@tauri-apps/api/core';

export const HammingDashboardScreen: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const filePath = location.state?.filePath;

  const [blockSize, setBlockSize] = useState('8');
  const [introduceErrors, setIntroduceErrors] = useState(false);
  const [generatedFiles, setGeneratedFiles] = useState<string[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  // Carga inicial y refresco de archivos desde el workspace
  const refreshWorkspace = async () => {
    try {
      const files = await invoke<string[]>('list_workspace_files');
      // Filtramos el archivo base (el original) para que no aparezca en "Generados"
      const baseName = filePath?.split(/[/\\]/).pop();
      const generated = files.filter(f => f !== baseName);
      setGeneratedFiles(generated);
      return generated;
    } catch (err) {
      console.error("Error al listar archivos del workspace:", err);
      return [];
    }
  };

  React.useEffect(() => {
    refreshWorkspace();
  }, []);

  if (!filePath) {
    return <Navigate to="/" replace />;
  }

  const getFileName = (path: string) => {
    return path.split(/[/\\]/).pop() || path;
  };

  const handleProtect = async () => {
    try {
      const blockSizeOpt = blockSize === '8' ? 1 : blockSize === '1024' ? 2 : 3;
      
      await invoke<string[]>('protect_file', { 
        path: filePath, 
        blockSizeOpt, 
        injectErrors: introduceErrors 
      });
      
      const newFiles = await refreshWorkspace();
      if (newFiles.length > 0) {
        setSelectedFile(newFiles[newFiles.length - 1]); // Seleccionar el más reciente
      }
    } catch (error) {
      console.error("Error during protection:", error);
      alert(`Error al proteger el archivo: ${error}`);
    }
  };

  const handleUnprotect = async () => {
    if (!selectedFile) return;
    try {
      const parentDir = filePath.substring(0, Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\')));
      const separator = filePath.includes('\\') ? '\\' : '/';
      const fullPathToSelected = `${parentDir}${separator}${selectedFile}`;

      const result = await invoke<string[]>('unprotect_file', { path: fullPathToSelected });
      
      await refreshWorkspace();
      
      if (result.length > 0) {
        setSelectedFile(result[0]);
        alert(`Archivo(s) desprotegido(s) con éxito:\n${result.join('\n')}`);
      }
    } catch (error) {
      console.error("Error during unprotection:", error);
      alert(`Error al desproteger el archivo: ${error}`);
    }
  };

  return (
    <div className="flex flex-col w-full max-w-6xl mx-auto py-6 h-full">
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
            <h1 className="text-3xl font-bold text-text-main tracking-tight">Hamming</h1>
            <span className="px-3 py-1 bg-primary/10 text-primary text-sm font-semibold rounded-full">
              {getFileName(filePath)}
            </span>
          </div>
        </div>
        
        <Button 
          variant="outline" 
          className="gap-2"
          onClick={() => navigate('/compare', { state: { filePath } })}
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
            <Shield className="text-primary" size={24} />
            <h2 className="text-xl font-semibold">Configuración de Protección</h2>
          </div>

          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Tamaño de Bloque</label>
              <div className="relative">
                <select 
                  className="w-full appearance-none bg-gray-50 border border-gray-200 text-gray-700 py-3 px-4 rounded-xl leading-tight focus:outline-none focus:bg-white focus:border-primary focus:ring-2 focus:ring-primary/20 transition-colors"
                  value={blockSize}
                  onChange={(e) => setBlockSize(e.target.value)}
                >
                  <option value="8">8 bits (.HA1)</option>
                  <option value="1024">1024 bits (.HA2)</option>
                  <option value="16384">16384 bits (.HA3)</option>
                </select>
                <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-4 text-gray-500">
                  <svg className="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
                </div>
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-3">¿Introducir errores en la transmisión?</label>
              <div className="flex gap-4">
                <label className={`flex-1 cursor-pointer rounded-xl border-2 p-4 flex items-center justify-center transition-all ${!introduceErrors ? 'border-primary bg-primary/5 text-primary font-semibold' : 'border-gray-200 text-gray-500 hover:border-gray-300'}`}>
                  <input type="radio" name="errors" className="hidden" checked={!introduceErrors} onChange={() => setIntroduceErrors(false)} />
                  <span className="flex items-center gap-2">{!introduceErrors && <CheckCircle size={18} />} NO</span>
                </label>
                <label className={`flex-1 cursor-pointer rounded-xl border-2 p-4 flex items-center justify-center transition-all ${introduceErrors ? 'border-red-500 bg-red-50 text-red-600 font-semibold' : 'border-gray-200 text-gray-500 hover:border-gray-300'}`}>
                  <input type="radio" name="errors" className="hidden" checked={introduceErrors} onChange={() => setIntroduceErrors(true)} />
                  <span className="flex items-center gap-2">{introduceErrors && <CheckCircle size={18} />} SÍ</span>
                </label>
              </div>
            </div>

            <div className="pt-4 mt-4 border-t border-gray-100">
              <Button onClick={handleProtect} fullWidth size="lg" className="gap-2 text-lg shadow-md">
                <Shield size={22} />
                PROTEGER
              </Button>
            </div>
          </div>
        </Card>

        {/* Right Column - Generated Files */}
        <Card className="p-6 lg:col-span-7 h-full flex flex-col">
          <div className="flex items-center gap-2 mb-6 border-b pb-4">
            <FileCode className="text-secondary" size={24} />
            <h2 className="text-xl font-semibold">Archivos Generados</h2>
          </div>

          {generatedFiles.length === 0 ? (
            <div className="flex-1 flex flex-col items-center justify-center text-gray-400 py-12">
              <FileCode size={48} className="mb-4 opacity-20" />
              <p>Configura y protege tu archivo para ver los resultados aquí.</p>
            </div>
          ) : (
            <div className="flex-1 flex flex-col">
              <div className="space-y-3 mb-6 flex-1">
                {generatedFiles.map((file, idx) => (
                  <div 
                    key={idx}
                    onClick={() => setSelectedFile(file)}
                    className={`p-4 rounded-xl border-2 cursor-pointer transition-all flex items-center justify-between
                      ${selectedFile === file ? 'border-secondary bg-secondary/5' : 'border-gray-100 hover:border-gray-200'}
                    `}
                  >
                    <div className="flex items-center gap-3">
                      <div className={`p-2 rounded-lg ${selectedFile === file ? 'bg-secondary/10 text-secondary' : 'bg-gray-100 text-gray-500'}`}>
                        <FileText size={20} />
                      </div>
                      <span className={`font-medium ${selectedFile === file ? 'text-secondary-hover' : 'text-text-main'}`}>
                        {file}
                      </span>
                    </div>
                    {selectedFile === file && <CheckCircle className="text-secondary" size={20} />}
                  </div>
                ))}
              </div>

              <div className="pt-4 border-t border-gray-100 mt-auto">
                <Button 
                  onClick={handleUnprotect} 
                  variant="secondary" 
                  fullWidth 
                  size="lg" 
                  className="gap-2 text-lg shadow-md"
                  disabled={!selectedFile}
                >
                  <Unlock size={22} />
                  DESPROTEGER
                </Button>
                <p className="text-xs text-center text-gray-400 mt-3">
                  La desprotección se aplicará sobre el archivo seleccionado en la lista.
                </p>
              </div>
            </div>
          )}
        </Card>
      </div>
    </div>
  );
};
