use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" height = "8" rx = "2" width = "20" x = "2" ry = "2" ></ rect > < rect height = "8" width = "20" ry = "2" rx = "2" x = "2" y = "14" ></ rect > < line x2 = "6.01" x1 = "6" y2 = "6" y1 = "6" ></ line > < line y2 = "18" x1 = "6" x2 = "6.01" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;