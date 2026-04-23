import React, { useState, useRef, useEffect, UIEvent } from 'react';
import { useLocation, useNavigate, Navigate } from 'react-router-dom';
import { ArrowLeft, SplitSquareHorizontal } from 'lucide-react';
import { Card } from '../components/ui/Card';

// MOCK: This represents the Rust function `highlight_error` that will be implemented later.
// It returns an array of objects indicating if a character is an error or not.
const mockHighlightError = async (text1: string, text2: string) => {
  // En una implementación real, esto llamaría a `invoke('highlight_error', { file1, file2 })`
  // Para propósitos de UI, comparamos los textos crudos si estuvieran cargados
  const result1 = [];
  const result2 = [];
  
  const maxLength = Math.max(text1.length, text2.length);
  
  for (let i = 0; i < maxLength; i++) {
    const char1 = text1[i] || ' ';
    const char2 = text2[i] || ' ';
    const isError = char1 !== char2;
    
    if (i < text1.length) result1.push({ char: char1, isError });
    if (i < text2.length) result2.push({ char: char2, isError });
  }
  
  return { result1, result2 };
};

export const ComparatorScreen: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { filePath, generatedFiles = [] } = location.state || {};

  const [file1, setFile1] = useState<string>(generatedFiles[0] || '');
  const [file2, setFile2] = useState<string>(generatedFiles[1] || generatedFiles[0] || '');
  
  const [content1, setContent1] = useState<{ char: string, isError: boolean }[]>([]);
  const [content2, setContent2] = useState<{ char: string, isError: boolean }[]>([]);

  const scrollRef1 = useRef<HTMLDivElement>(null);
  const scrollRef2 = useRef<HTMLDivElement>(null);
  const isScrollingRef = useRef<'none' | 'left' | 'right'>('none');

  if (!filePath) {
    return <Navigate to="/" replace />;
  }

  const allAvailableFiles = [filePath.split(/[/\\]/).pop(), ...generatedFiles].filter(Boolean) as string[];

  // Simula la carga de archivos y comparación
  useEffect(() => {
    const loadAndCompare = async () => {
      // MOCK: En una app real, leeríamos el contenido de los archivos seleccionados usando Tauri FS.
      // Aquí simulamos contenidos con diferencias para mostrar la UI.
      let text1 = "Este es un archivo de prueba.\nContiene multiples lineas para probar el scroll.\nEl codigo de Hamming es genial.";
      let text2 = "Este es un archivo de prueba.\nContiene multiqles lineas para probar el scroll.\nEl codigo de Hamming es lenial.";
      
      // Solo para que los mock sean un poco diferentes según el archivo elegido
      if (file1 === file2) {
        text2 = text1; 
      }

      const { result1, result2 } = await mockHighlightError(text1, text2);
      setContent1(result1);
      setContent2(result2);
    };

    loadAndCompare();
  }, [file1, file2]);

  const handleScroll = (e: UIEvent<HTMLDivElement>, source: 'left' | 'right') => {
    if (isScrollingRef.current !== 'none' && isScrollingRef.current !== source) {
      return;
    }
    
    isScrollingRef.current = source;
    
    const target = e.currentTarget;
    const otherRef = source === 'left' ? scrollRef2 : scrollRef1;
    
    if (otherRef.current) {
      // Sync scroll percentage instead of exact pixels in case heights differ slightly
      const scrollPercentage = target.scrollTop / (target.scrollHeight - target.clientHeight);
      
      // Calculate new scroll position for the other container
      const newScrollTop = scrollPercentage * (otherRef.current.scrollHeight - otherRef.current.clientHeight);
      
      if (Math.abs(otherRef.current.scrollTop - newScrollTop) > 1) {
        otherRef.current.scrollTop = newScrollTop;
      }
    }
    
    // Clear the scroll lock after a small timeout
    setTimeout(() => {
      isScrollingRef.current = 'none';
    }, 50);
  };

  return (
    <div className="flex flex-col w-full max-w-7xl mx-auto h-[90vh] py-4">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <button 
            onClick={() => navigate('/hamming', { state: { filePath } })}
            className="flex items-center text-text-muted hover:text-text-main transition-colors mb-2 font-medium"
          >
            <ArrowLeft size={18} className="mr-1" />
            Volver al Dashboard
          </button>
          <div className="flex items-center gap-2">
            <SplitSquareHorizontal className="text-primary" size={28} />
            <h1 className="text-2xl font-bold text-text-main tracking-tight">Comparador de Archivos</h1>
          </div>
        </div>
      </div>

      <div className="flex-1 grid grid-cols-1 md:grid-cols-2 gap-6 min-h-0">
        {/* Left Column */}
        <Card className="flex flex-col h-full bg-white">
          <div className="p-4 border-b border-gray-100 bg-gray-50/50">
            <select 
              className="w-full bg-white border border-gray-200 text-gray-700 py-2 px-3 rounded-lg focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary/50 text-sm font-medium"
              value={file1}
              onChange={(e) => setFile1(e.target.value)}
            >
              {allAvailableFiles.map((f, i) => (
                <option key={i} value={f}>{f}</option>
              ))}
            </select>
          </div>
          <div 
            ref={scrollRef1}
            onScroll={(e) => handleScroll(e, 'left')}
            className="flex-1 p-6 overflow-y-auto font-mono text-sm leading-relaxed whitespace-pre-wrap break-words"
          >
            {content1.map((item, idx) => (
              <span 
                key={idx} 
                className={item.isError ? 'bg-red-100 text-red-700 px-[1px] font-bold rounded-sm border-b border-red-300' : ''}
              >
                {item.char}
              </span>
            ))}
          </div>
        </Card>

        {/* Right Column */}
        <Card className="flex flex-col h-full bg-white">
          <div className="p-4 border-b border-gray-100 bg-gray-50/50">
            <select 
              className="w-full bg-white border border-gray-200 text-gray-700 py-2 px-3 rounded-lg focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary/50 text-sm font-medium"
              value={file2}
              onChange={(e) => setFile2(e.target.value)}
            >
              {allAvailableFiles.map((f, i) => (
                <option key={i} value={f}>{f}</option>
              ))}
            </select>
          </div>
          <div 
            ref={scrollRef2}
            onScroll={(e) => handleScroll(e, 'right')}
            className="flex-1 p-6 overflow-y-auto font-mono text-sm leading-relaxed whitespace-pre-wrap break-words"
          >
            {content2.map((item, idx) => (
              <span 
                key={idx} 
                className={item.isError ? 'bg-red-100 text-red-700 px-[1px] font-bold rounded-sm border-b border-red-300' : ''}
              >
                {item.char}
              </span>
            ))}
          </div>
        </Card>
      </div>
    </div>
  );
};
