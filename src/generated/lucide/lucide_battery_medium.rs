use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" y = "7" width = "16" ry = "2" height = "10" rx = "2" ></ rect > < line x2 = "22" y2 = "13" x1 = "22" y1 = "11" ></ line > < line y2 = "13" x2 = "6" x1 = "6" y1 = "11" ></ line > < line y1 = "11" x2 = "10" y2 = "13" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;