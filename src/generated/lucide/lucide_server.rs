use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" x = "2" width = "20" rx = "2" y = "2" ry = "2" ></ rect > < rect width = "20" ry = "2" height = "8" rx = "2" x = "2" y = "14" ></ rect > < line y1 = "6" x1 = "6" y2 = "6" x2 = "6.01" ></ line > < line x2 = "6.01" y2 = "18" x1 = "6" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24")] } ;