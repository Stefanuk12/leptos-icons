use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" y = "2" x = "2" height = "8" rx = "2" ry = "2" ></ rect > < rect y = "14" ry = "2" width = "20" height = "8" x = "2" rx = "2" ></ rect > < line x1 = "6" y1 = "6" x2 = "6.01" y2 = "6" ></ line > < line x2 = "6.01" y1 = "18" x1 = "6" y2 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;