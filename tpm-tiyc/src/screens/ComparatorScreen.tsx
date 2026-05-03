import React, { useState, useRef, useEffect, UIEvent } from 'react';
import { useLocation, useNavigate, Navigate } from 'react-router-dom';
import { ArrowLeft, SplitSquareHorizontal, ChevronLeft, ChevronRight } from 'lucide-react';
import { Card } from '../components/ui/Card';
import { invoke } from '@tauri-apps/api/core';

// Compare original txt file with encoded file (Hamming) and highlight the differences.
const compareFiles = (text1: string, text2: string, enableHighlight: boolean) => {
  const result1 = [];
  const result2 = [];
  
  const maxLength = Math.max(text1.length, text2.length);
  
  for (let i = 0; i < maxLength; i++) {
    const char1 = text1[i] || ' ';
    const char2 = text2[i] || ' ';
    const isError = enableHighlight && (char1 !== char2);
    
    if (i < text1.length) result1.push({ char: char1, isError });
    if (i < text2.length) result2.push({ char: char2, isError });
  }
  
  return { result1, result2 };
};

export const ComparatorScreen: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { filePath, sourceScreen = '/hamming' } = location.state || {};

  // Route guard.
  if (!filePath) {
    return <Navigate to="/" replace />;
  }

  const [allAvailableFiles, setAllAvailableFiles] = useState<string[]>([]);
  const [file1, setFile1] = useState<string>('');
  const [file2, setFile2] = useState<string>('');
  
  useEffect(() => {
    const fetchFiles = async () => {
      try {
        const files = await invoke<string[]>('list_workspace_files');
        setAllAvailableFiles(files);
        if (files.length > 0) {
          // Original txt file is set as default in file1
          const originalName = filePath.split(/[/\\]/).pop();
          const default1 = files.includes(originalName || '') ? originalName || files[0] : files[0];
          setFile1(default1);
          setFile2(files.length > 1 ? files[1] : files[0]);
        }
      } catch (err) {
        console.error("Error fetching workspace files:", err);
      }
    };
    fetchFiles();
  }, [filePath]);
  
  const [content1, setContent1] = useState<{ char: string, isError: boolean }[]>([]);
  const [content2, setContent2] = useState<{ char: string, isError: boolean }[]>([]);

  const [fullText1, setFullText1] = useState('');
  const [fullText2, setFullText2] = useState('');
  const [currentPage, setCurrentPage] = useState(0);
  const PAGE_SIZE = 10000;

  const scrollRef1 = useRef<HTMLDivElement>(null);
  const scrollRef2 = useRef<HTMLDivElement>(null);
  const isScrollingRef = useRef<'none' | 'left' | 'right'>('none');

  // Loads files into full text state
  useEffect(() => {
    const loadFiles = async () => {
      let t1 = "";
      let t2 = "";

      const parentDir = filePath.substring(0, Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\')));
      const separator = filePath.includes('\\') ? '\\' : '/';

      try {
        if (file1) {
          const fullPath1 = file1 === filePath.split(/[/\\]/).pop() ? filePath : `${parentDir}${separator}${file1}`;
          t1 = await invoke<string>('read_file_content', { path: fullPath1 });
        }
        
        if (file2) {
          const fullPath2 = file2 === filePath.split(/[/\\]/).pop() ? filePath : `${parentDir}${separator}${file2}`;
          t2 = await invoke<string>('read_file_content', { path: fullPath2 });
        }
      } catch (error) {
        console.error("Error reading files for comparison:", error);
        t1 = file1 ? `[Error leyendo el archivo: ${error}]` : "";
        t2 = file2 ? `[Error leyendo el archivo: ${error}]` : "";
      }

      setFullText1(t1);
      setFullText2(t2);
      setCurrentPage(0); // Reset page on file change
    };

    loadFiles();
  }, [file1, file2, filePath]);

  // Slices and compares the current page
  useEffect(() => {
    const isHammingFile = (filename: string) => {
      const ext = filename.split('.').pop()?.toUpperCase() || '';
      return ext.startsWith('HA') || ext.startsWith('HE');
    };

    const enableHighlight = !isHammingFile(file1) && !isHammingFile(file2);
    
    const start = currentPage * PAGE_SIZE;
    const end = start + PAGE_SIZE;
    
    const slice1 = fullText1.slice(start, end);
    const slice2 = fullText2.slice(start, end);

    const { result1, result2 } = compareFiles(slice1, slice2, enableHighlight);
    setContent1(result1);
    setContent2(result2);
    
    // Reset scroll positions when page changes
    if (scrollRef1.current) scrollRef1.current.scrollTop = 0;
    if (scrollRef2.current) scrollRef2.current.scrollTop = 0;
  }, [fullText1, fullText2, file1, file2, currentPage]);

  const handleScroll = (e: UIEvent<HTMLDivElement>, source: 'left' | 'right') => {
    if (isScrollingRef.current !== 'none' && isScrollingRef.current !== source) {
      return;
    }
    
    // Sync scroll position between the two files (no infinite loop)
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
            onClick={() => navigate(sourceScreen, { state: { filePath } })}
            className="flex items-center text-text-muted hover:text-text-main transition-colors mb-2 font-medium"
          >
            <ArrowLeft size={18} className="mr-1" />
            Volver a {sourceScreen === '/huffman' ? 'Huffman' : 'Hamming'}
          </button>
          <div className="flex items-center gap-2">
            <SplitSquareHorizontal className="text-primary" size={28} />
            <h1 className="text-2xl font-bold text-text-main tracking-tight">Comparador de Archivos</h1>
          </div>
        </div>
        
        {/* Pagination Controls */}
        <div className="flex items-center gap-4 bg-white px-4 py-2 rounded-lg shadow-sm border border-gray-100">
          <button 
            onClick={() => setCurrentPage(p => Math.max(0, p - 1))}
            disabled={currentPage === 0}
            className="p-1 rounded hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed text-gray-600 transition-colors"
          >
            <ChevronLeft size={20} />
          </button>
          <span className="text-sm font-medium text-gray-600 min-w-[100px] text-center">
            Página {currentPage + 1} de {Math.max(1, Math.ceil(Math.max(fullText1.length, fullText2.length) / PAGE_SIZE))}
          </span>
          <button 
            onClick={() => setCurrentPage(p => p + 1)}
            disabled={(currentPage + 1) * PAGE_SIZE >= Math.max(fullText1.length, fullText2.length)}
            className="p-1 rounded hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed text-gray-600 transition-colors"
          >
            <ChevronRight size={20} />
          </button>
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
