use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" rx = "2" ry = "2" width = "20" x = "2" height = "8" ></ rect > < rect y = "14" rx = "2" ry = "2" width = "20" height = "8" x = "2" ></ rect > < line y1 = "6" y2 = "6" x2 = "6.01" x1 = "6" ></ line > < line x2 = "6.01" x1 = "6" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;