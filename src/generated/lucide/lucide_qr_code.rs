use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "1" width = "5" height = "5" ></ rect > < rect width = "5" height = "5" y = "3" rx = "1" x = "16" ></ rect > < rect height = "5" y = "16" width = "5" rx = "1" x = "3" ></ rect > < path d = "M21 16h-3a2 2 0 0 0-2 2v3" ></ path > < path d = "M21 21v.01" ></ path > < path d = "M12 7v3a2 2 0 0 1-2 2H7" ></ path > < path d = "M3 12h.01" ></ path > < path d = "M12 3h.01" ></ path > < path d = "M12 16v.01" ></ path > < path d = "M16 12h1" ></ path > < path d = "M21 12v.01" ></ path > < path d = "M12 21v-1" ></ path > < / > } } pub const LUCIDE_QR_CODE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;