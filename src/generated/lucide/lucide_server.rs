use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" width = "20" x = "2" y = "2" height = "8" ></ rect > < rect x = "2" width = "20" height = "8" ry = "2" y = "14" rx = "2" ></ rect > < line x1 = "6" x2 = "6.01" y1 = "6" y2 = "6" ></ line > < line x1 = "6" y2 = "18" y1 = "18" x2 = "6.01" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;