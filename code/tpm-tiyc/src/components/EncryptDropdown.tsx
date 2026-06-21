import React, { useState, useRef, useEffect } from 'react';
import { Lock, Calendar, CheckSquare, Square } from 'lucide-react';
import { Button } from './ui/Button';
import { Card } from './ui/Card';
import { invoke } from '@tauri-apps/api/core';
import { CustomCalendar } from './ui/CustomCalendar';

interface EncryptDropdownProps {
  originalFilePath: string;
}

export const EncryptDropdown: React.FC<EncryptDropdownProps> = ({ originalFilePath }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedDate, setSelectedDate] = useState<string>('');
  const [isExact, setIsExact] = useState<boolean>(true);
  const [isConfigured, setIsConfigured] = useState<boolean>(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);


  const handleEncrypt = async () => {
    if (!selectedDate) return;
    
    try {
      await invoke('set_encryption_settings', {
        originalPath: originalFilePath,
        targetDate: selectedDate,
        strictDate: isExact
      });
      setIsConfigured(true);
      setIsOpen(false);
    } catch (err) {
      console.error('Error setting encryption:', err);
      alert('Error al configurar la encriptación.');
    }
  };

  const handleCancelEncryption = async () => {
    try {
      await invoke('clear_encryption_settings');
      setIsConfigured(false);
      setSelectedDate('');
      setIsExact(true);
      setIsOpen(false);
    } catch (err) {
      console.error('Error clearing encryption:', err);
    }
  };

  return (
    <div className="relative" ref={dropdownRef}>
      <Button 
        onClick={() => setIsOpen(!isOpen)} 
        variant={isConfigured ? "primary" : "secondary"}
        className="gap-2 flex items-center shadow-sm"
      >
        <Lock size={18} />
        {isConfigured ? "Encriptación Pendiente" : "Encriptar"}
      </Button>

      {isOpen && (
        <Card className="absolute top-full left-0 mt-2 w-80 p-5 z-50 shadow-xl border border-gray-100 flex flex-col gap-4 animate-in fade-in slide-in-from-top-2">
          {isConfigured ? (
            <div className="flex flex-col gap-3">
              <p className="text-sm text-text-muted">
                El archivo se encriptará cuando cierres la aplicación o vuelvas a cargar otro archivo.
              </p>
              <div className="bg-gray-50 p-3 rounded-lg border border-gray-100">
                <p className="text-sm font-medium text-text-main">
                  Fecha de apertura: {selectedDate}
                </p>
                <p className="text-xs text-text-muted mt-1">
                  {isExact ? "Solo en esta fecha exacta" : "A partir de esta fecha"}
                </p>
              </div>
              <Button onClick={handleCancelEncryption} variant="danger" className="w-full mt-2">
                Cancelar Encriptación
              </Button>
            </div>
          ) : (
            <>
              <div className="flex flex-col gap-1.5">
                <label className="text-sm font-medium text-text-main flex items-center gap-1.5">
                  <Calendar size={16} className="text-primary" />
                  Fecha de apertura
                </label>
                <div className="flex justify-center w-full my-2">
                  <CustomCalendar 
                    selectedDate={selectedDate} 
                    onSelectDate={setSelectedDate} 
                    isExact={isExact} 
                  />
                </div>
              </div>

              <button
                className="flex items-start gap-2 text-left group"
                onClick={() => setIsExact(!isExact)}
              >
                <div className="mt-0.5 text-primary">
                  {isExact ? <CheckSquare size={18} /> : <Square size={18} className="text-gray-300 group-hover:text-primary/50" />}
                </div>
                <span className="text-sm text-text-muted group-hover:text-text-main transition-colors leading-tight">
                  Permitir apertura <strong>solo en esta fecha exacta</strong> (si se desmarca, podrá abrirse en esta fecha o posteriores).
                </span>
              </button>

              <Button 
                onClick={handleEncrypt} 
                disabled={!selectedDate}
                className="w-full mt-2"
              >
                Confirmar Encriptación
              </Button>
            </>
          )}
        </Card>
      )}
    </div>
  );
};
