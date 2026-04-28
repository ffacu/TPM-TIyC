
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { MainLayout } from './layouts/MainLayout';
import { LoadFileScreen } from './screens/LoadFileScreen';
import { HomeScreen } from './screens/HomeScreen';
import { HammingDashboardScreen } from './screens/HammingDashboardScreen';
import { ComparatorScreen } from './screens/ComparatorScreen';

//navigation app
export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<LoadFileScreen />} />
          <Route path="home" element={<HomeScreen />} />
          <Route path="hamming" element={<HammingDashboardScreen />} />
          <Route path="compare" element={<ComparatorScreen />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
