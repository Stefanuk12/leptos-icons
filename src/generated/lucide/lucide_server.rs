use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" rx = "2" x = "2" ry = "2" y = "2" height = "8" ></ rect > < rect x = "2" ry = "2" rx = "2" height = "8" width = "20" y = "14" ></ rect > < line x1 = "6" y1 = "6" y2 = "6" x2 = "6.01" ></ line > < line y2 = "18" x2 = "6.01" x1 = "6" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24")] } ;