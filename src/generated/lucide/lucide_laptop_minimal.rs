use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "4" width = "18" rx = "2" ry = "2" height = "12" ></ rect > < line y1 = "20" y2 = "20" x1 = "2" x2 = "22" ></ line > < / > } } pub const LUCIDE_LAPTOP_MINIMAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;