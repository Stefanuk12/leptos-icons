use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" height = "8" y = "2" rx = "2" ry = "2" ></ rect > < rect ry = "2" x = "2" height = "8" width = "20" y = "14" rx = "2" ></ rect > < line x2 = "6.01" y2 = "6" y1 = "6" x1 = "6" ></ line > < line y1 = "18" x2 = "6.01" x1 = "6" y2 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;