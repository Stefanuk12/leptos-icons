use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" width = "20" x = "2" rx = "2" ry = "2" y = "2" ></ rect > < rect width = "20" y = "14" rx = "2" x = "2" ry = "2" height = "8" ></ rect > < line x1 = "6" y1 = "6" x2 = "6.01" y2 = "6" ></ line > < line y1 = "18" x2 = "6.01" x1 = "6" y2 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;