use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" height = "8" width = "20" y = "2" rx = "2" ry = "2" ></ rect > < rect x = "2" width = "20" height = "8" y = "14" ry = "2" rx = "2" ></ rect > < line x2 = "6.01" y2 = "6" x1 = "6" y1 = "6" ></ line > < line y2 = "18" x1 = "6" x2 = "6.01" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none")] } ;