use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "8" rx = "2" y = "2" x = "2" ry = "2" ></ rect > < rect rx = "2" y = "14" ry = "2" width = "20" x = "2" height = "8" ></ rect > < line x1 = "6" y2 = "6" x2 = "6.01" y1 = "6" ></ line > < line y2 = "18" x2 = "6.01" x1 = "6" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;