use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" width = "16" ry = "2" height = "10" rx = "2" x = "2" ></ rect > < line x2 = "22" y1 = "11" y2 = "13" x1 = "22" ></ line > < line y1 = "11" y2 = "13" x1 = "6" x2 = "6" ></ line > < line x2 = "10" y1 = "11" y2 = "13" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;