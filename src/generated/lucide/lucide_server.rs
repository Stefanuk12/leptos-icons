use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" height = "8" width = "20" x = "2" y = "2" ></ rect > < rect x = "2" width = "20" height = "8" y = "14" ry = "2" rx = "2" ></ rect > < line y1 = "6" x2 = "6.01" y2 = "6" x1 = "6" ></ line > < line x1 = "6" y2 = "18" x2 = "6.01" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;