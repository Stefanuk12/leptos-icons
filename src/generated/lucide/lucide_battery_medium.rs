use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" ry = "2" height = "10" y = "7" width = "16" rx = "2" ></ rect > < line x2 = "22" x1 = "22" y2 = "13" y1 = "11" ></ line > < line y2 = "13" x1 = "6" x2 = "6" y1 = "11" ></ line > < line x2 = "10" y1 = "11" x1 = "10" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;