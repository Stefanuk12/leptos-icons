use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "20" height = "8" ry = "2" rx = "2" y = "2" ></ rect > < rect height = "8" y = "14" ry = "2" rx = "2" x = "2" width = "20" ></ rect > < line x2 = "6.01" y1 = "6" x1 = "6" y2 = "6" ></ line > < line x1 = "6" x2 = "6.01" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24")] } ;