use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "8" y = "2" rx = "2" ry = "2" x = "2" ></ rect > < rect ry = "2" x = "2" width = "20" y = "14" rx = "2" height = "8" ></ rect > < line x2 = "6.01" y2 = "6" x1 = "6" y1 = "6" ></ line > < line y2 = "18" y1 = "18" x1 = "6" x2 = "6.01" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;