use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" rx = "2" y = "2" width = "20" x = "2" ry = "2" ></ rect > < rect rx = "2" height = "8" y = "14" width = "20" ry = "2" x = "2" ></ rect > < line x2 = "6.01" x1 = "6" y1 = "6" y2 = "6" ></ line > < line x1 = "6" x2 = "6.01" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;