use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" rx = "2" y = "2" ry = "2" width = "20" x = "2" ></ rect > < rect y = "14" rx = "2" width = "20" ry = "2" height = "8" x = "2" ></ rect > < line x1 = "6" y2 = "6" x2 = "6.01" y1 = "6" ></ line > < line y1 = "18" y2 = "18" x1 = "6" x2 = "6.01" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;