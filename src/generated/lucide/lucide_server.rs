use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" x = "2" y = "2" height = "8" width = "20" rx = "2" ></ rect > < rect height = "8" ry = "2" rx = "2" width = "20" x = "2" y = "14" ></ rect > < line y1 = "6" y2 = "6" x1 = "6" x2 = "6.01" ></ line > < line y2 = "18" y1 = "18" x2 = "6.01" x1 = "6" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round")] } ;