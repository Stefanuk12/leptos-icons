use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" height = "8" y = "2" rx = "2" ry = "2" width = "20" ></ rect > < rect ry = "2" height = "8" x = "2" rx = "2" y = "14" width = "20" ></ rect > < line y2 = "6" x2 = "6.01" x1 = "6" y1 = "6" ></ line > < line x1 = "6" x2 = "6.01" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;