use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" rx = "2" ry = "2" y = "7" height = "10" width = "16" ></ rect > < line y2 = "13" y1 = "11" x1 = "22" x2 = "22" ></ line > < / > } } pub const LUCIDE_BATTERY : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;