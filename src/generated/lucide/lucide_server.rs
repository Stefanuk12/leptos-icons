use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" y = "2" width = "20" x = "2" rx = "2" height = "8" ></ rect > < rect height = "8" x = "2" width = "20" ry = "2" y = "14" rx = "2" ></ rect > < line y2 = "6" x1 = "6" y1 = "6" x2 = "6.01" ></ line > < line x2 = "6.01" x1 = "6" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_SERVER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor")] } ;