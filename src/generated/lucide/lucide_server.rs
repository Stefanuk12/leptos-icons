use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "8" y = "2" ry = "2" rx = "2" x = "2" ></ rect > < rect width = "20" height = "8" y = "14" ry = "2" x = "2" rx = "2" ></ rect > < line y2 = "6" y1 = "6" x2 = "6.01" x1 = "6" ></ line > < line x1 = "6" x2 = "6.01" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;