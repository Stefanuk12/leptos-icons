use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" width = "20" x = "2" rx = "2" ry = "2" y = "2" ></ rect > < rect x = "2" height = "8" width = "20" y = "14" rx = "2" ry = "2" ></ rect > < line x1 = "6" y2 = "6" y1 = "6" x2 = "6.01" ></ line > < line y2 = "18" y1 = "18" x1 = "6" x2 = "6.01" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;