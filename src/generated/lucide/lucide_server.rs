use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" ry = "2" y = "2" width = "20" x = "2" rx = "2" ></ rect > < rect x = "2" rx = "2" height = "8" y = "14" ry = "2" width = "20" ></ rect > < line y2 = "6" x1 = "6" x2 = "6.01" y1 = "6" ></ line > < line x1 = "6" x2 = "6.01" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;