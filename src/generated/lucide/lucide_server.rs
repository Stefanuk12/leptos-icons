use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "2" height = "8" width = "20" y = "2" ry = "2" ></ rect > < rect rx = "2" ry = "2" y = "14" x = "2" width = "20" height = "8" ></ rect > < line y2 = "6" y1 = "6" x1 = "6" x2 = "6.01" ></ line > < line y2 = "18" x1 = "6" y1 = "18" x2 = "6.01" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;