use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "5" y = "3" rx = "1" x = "3" width = "5" ></ rect > < rect width = "5" y = "3" x = "16" rx = "1" height = "5" ></ rect > < rect x = "3" y = "16" width = "5" rx = "1" height = "5" ></ rect > < path d = "M21 16h-3a2 2 0 0 0-2 2v3" ></ path > < path d = "M21 21v.01" ></ path > < path d = "M12 7v3a2 2 0 0 1-2 2H7" ></ path > < path d = "M3 12h.01" ></ path > < path d = "M12 3h.01" ></ path > < path d = "M12 16v.01" ></ path > < path d = "M16 12h1" ></ path > < path d = "M21 12v.01" ></ path > < path d = "M12 21v-1" ></ path > < / > } } pub const LUCIDE_QR_CODE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("height" , "24")] } ;