use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "20" rx = "2" x = "2" height = "8" y = "2" ></ rect > < rect width = "20" rx = "2" y = "14" height = "8" ry = "2" x = "2" ></ rect > < line x2 = "6.01" y1 = "6" y2 = "6" x1 = "6" ></ line > < line y2 = "18" y1 = "18" x1 = "6" x2 = "6.01" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;