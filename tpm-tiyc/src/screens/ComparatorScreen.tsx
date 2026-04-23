import React, { useState, useRef, useEffect, UIEvent } from 'react';
import { useLocation, useNavigate, Navigate } from 'react-router-dom';
import { ArrowLeft, SplitSquareHorizontal } from 'lucide-react';
import { Card } from '../components/ui/Card';
import { invoke } from '@tauri-apps/api/core';

// Compara dos textos carácter por carácter y devuelve el formato para renderizar.
const compareFiles = (text1: string, text2: string) => {
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

  if (!filePath) {
    return <Navigate to="/" replace />;
  }

  // Filtramos los archivos generados para excluir los binarios (.HAx, .HEx) ya que no son legibles en texto plano.
  const allAvailableFiles = [filePath.split(/[/\\]/).pop(), ...generatedFiles]
    .filter(Boolean)
    .filter(f => {
      const ext = f?.split('.').pop()?.toUpperCase() || '';
      return !ext.startsWith('HA') && !ext.startsWith('HE');
    }) as string[];

  const [file1, setFile1] = useState<string>(allAvailableFiles[0] || '');
  const [file2, setFile2] = useState<string>(allAvailableFiles[1] || allAvailableFiles[0] || '');
  
  const [content1, setContent1] = useState<{ char: string, isError: boolean }[]>([]);
  const [content2, setContent2] = useState<{ char: string, isError: boolean }[]>([]);

  const scrollRef1 = useRef<HTMLDivElement>(null);
  const scrollRef2 = useRef<HTMLDivElement>(null);
  const isScrollingRef = useRef<'none' | 'left' | 'right'>('none');

  // Carga y comparación real de archivos
  useEffect(() => {
    const loadAndCompare = async () => {
      let text1 = "";
      let text2 = "";

      const parentDir = filePath.substring(0, Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\')));
      const separator = filePath.includes('\\') ? '\\' : '/';

      try {
        if (file1) {
          const fullPath1 = file1 === filePath.split(/[/\\]/).pop() ? filePath : `${parentDir}${separator}${file1}`;
          text1 = await invoke<string>('read_file_content', { path: fullPath1 });
        }
        
        if (file2) {
          const fullPath2 = file2 === filePath.split(/[/\\]/).pop() ? filePath : `${parentDir}${separator}${file2}`;
          text2 = await invoke<string>('read_file_content', { path: fullPath2 });
        }
      } catch (error) {
        console.error("Error reading files for comparison:", error);
        text1 = file1 ? `[Error leyendo el archivo: ${error}]` : "";
        text2 = file2 ? `[Error leyendo el archivo: ${error}]` : "";
      }

      const { result1, result2 } = compareFiles(text1, text2);
      setContent1(result1);
      setContent2(result2);
    };

    loadAndCompare();
  }, [file1, file2, filePath]);

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
