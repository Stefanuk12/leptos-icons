use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" x = "2" ry = "2" height = "10" width = "16" rx = "2" ></ rect > < line x1 = "22" y1 = "11" x2 = "22" y2 = "13" ></ line > < line x1 = "6" x2 = "6" y2 = "13" y1 = "11" ></ line > < line y1 = "11" x2 = "10" y2 = "13" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;