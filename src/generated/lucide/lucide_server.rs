use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" ry = "2" rx = "2" height = "8" y = "2" x = "2" ></ rect > < rect x = "2" ry = "2" rx = "2" width = "20" y = "14" height = "8" ></ rect > < line y1 = "6" x1 = "6" y2 = "6" x2 = "6.01" ></ line > < line y2 = "18" x1 = "6" y1 = "18" x2 = "6.01" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;