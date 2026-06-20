import React, { useState } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';

interface CustomCalendarProps {
  selectedDate: string;
  onSelectDate: (date: string) => void;
  isExact: boolean;
}

export const CustomCalendar: React.FC<CustomCalendarProps> = ({ selectedDate, onSelectDate, isExact }) => {
  const [viewDate, setViewDate] = useState(() => {
    if (selectedDate) {
      const parts = selectedDate.split('-');
      return new Date(parseInt(parts[0]), parseInt(parts[1]) - 1, parseInt(parts[2]));
    }
    return new Date();
  });

  const today = new Date();
  today.setHours(0, 0, 0, 0);

  const selected = selectedDate ? new Date(selectedDate + 'T00:00:00') : null;
  if (selected) selected.setHours(0, 0, 0, 0);

  const startOfMonth = new Date(viewDate.getFullYear(), viewDate.getMonth(), 1);
  const endOfMonth = new Date(viewDate.getFullYear(), viewDate.getMonth() + 1, 0);

  const daysInMonth = endOfMonth.getDate();
  const startDayOfWeek = startOfMonth.getDay(); // 0 is Sunday

  const isPrevMonthDisabled = viewDate.getFullYear() === today.getFullYear() && viewDate.getMonth() <= today.getMonth() || viewDate.getFullYear() < today.getFullYear();

  const handlePrevMonth = () => {
    if (!isPrevMonthDisabled) {
      setViewDate(new Date(viewDate.getFullYear(), viewDate.getMonth() - 1, 1));
    }
  };

  const handleNextMonth = () => {
    setViewDate(new Date(viewDate.getFullYear(), viewDate.getMonth() + 1, 1));
  };

  const handleDayClick = (day: number) => {
    const d = new Date(viewDate.getFullYear(), viewDate.getMonth(), day);
    if (d < today) return; // Prevent picking past days
    
    // Format as YYYY-MM-DD
    const yyyy = d.getFullYear();
    const mm = String(d.getMonth() + 1).padStart(2, '0');
    const dd = String(d.getDate()).padStart(2, '0');
    onSelectDate(`${yyyy}-${mm}-${dd}`);
  };

  const days = [];
  for (let i = 0; i < startDayOfWeek; i++) {
    days.push(<div key={`empty-${i}`} className="w-8 h-8"></div>);
  }

  for (let d = 1; d <= daysInMonth; d++) {
    const currentDate = new Date(viewDate.getFullYear(), viewDate.getMonth(), d);
    currentDate.setHours(0, 0, 0, 0);

    const isPast = currentDate < today;
    const isSelected = selected && currentDate.getTime() === selected.getTime();
    
    // Visualization logic
    let isGrayscale = false;
    let isHighlight = false;

    if (isPast) {
      isGrayscale = true;
    } else if (selected) {
      if (isExact) {
        // Only the selected date is highlighted, rest is grayscale
        isGrayscale = !isSelected;
        isHighlight = !!isSelected;
      } else {
        // Selected date and future dates are highlighted, past dates (relative to selected) are grayscale
        if (currentDate < selected) {
          isGrayscale = true;
        } else {
          isHighlight = true;
        }
      }
    } else {
      // Default: nothing selected, only past system days are grayscale
      isGrayscale = isPast;
    }

    days.push(
      <button
        key={d}
        disabled={isPast}
        onClick={(e) => {
          e.preventDefault();
          e.stopPropagation();
          handleDayClick(d);
        }}
        className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors
          ${isPast ? 'text-gray-300 cursor-not-allowed' : 'cursor-pointer'}
          ${isSelected && isExact ? 'bg-primary text-white font-bold ring-2 ring-primary ring-offset-1' : ''}
          ${isSelected && !isExact ? 'bg-primary text-white font-bold' : ''}
          ${!isSelected && isHighlight ? 'bg-primary/20 text-primary' : ''}
          ${!isSelected && !isHighlight && !isGrayscale ? 'hover:bg-gray-100 text-gray-700' : ''}
          ${!isSelected && isGrayscale && !isPast ? 'text-gray-400 bg-gray-50' : ''}
        `}
      >
        {d}
      </button>
    );
  }

  const monthNames = ["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio", "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"];

  return (
    <div className="bg-white border border-gray-200 rounded-lg p-3 shadow-sm select-none" onClick={(e) => e.stopPropagation()}>
      <div className="flex justify-between items-center mb-2">
        <button 
          onClick={(e) => { e.preventDefault(); e.stopPropagation(); handlePrevMonth(); }} 
          disabled={isPrevMonthDisabled}
          className={`p-1 rounded-md ${isPrevMonthDisabled ? 'text-gray-300 cursor-not-allowed' : 'text-gray-600 hover:bg-gray-100'}`}
        >
          <ChevronLeft size={18} />
        </button>
        <div className="font-semibold text-gray-800 text-sm">
          {monthNames[viewDate.getMonth()]} {viewDate.getFullYear()}
        </div>
        <button 
          onClick={(e) => { e.preventDefault(); e.stopPropagation(); handleNextMonth(); }}
          className="p-1 rounded-md text-gray-600 hover:bg-gray-100"
        >
          <ChevronRight size={18} />
        </button>
      </div>
      <div className="grid grid-cols-7 gap-1 text-center mb-1">
        {['Do', 'Lu', 'Ma', 'Mi', 'Ju', 'Vi', 'Sa'].map(day => (
          <div key={day} className="text-xs font-semibold text-gray-400">{day}</div>
        ))}
      </div>
      <div className="grid grid-cols-7 gap-1 place-items-center">
        {days}
      </div>
    </div>
  );
};
