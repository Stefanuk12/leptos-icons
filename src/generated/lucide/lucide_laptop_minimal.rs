use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" x = "3" height = "12" width = "18" rx = "2" ry = "2" ></ rect > < line y2 = "20" x2 = "22" y1 = "20" x1 = "2" ></ line > < / > } } pub const LUCIDE_LAPTOP_MINIMAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;