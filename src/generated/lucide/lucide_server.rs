use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" width = "20" rx = "2" y = "2" ry = "2" x = "2" ></ rect > < rect x = "2" y = "14" ry = "2" height = "8" width = "20" rx = "2" ></ rect > < line x1 = "6" x2 = "6.01" y1 = "6" y2 = "6" ></ line > < line x2 = "6.01" y1 = "18" x1 = "6" y2 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;