use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" rx = "2" x = "2" ry = "2" y = "7" height = "10" ></ rect > < line y1 = "11" y2 = "13" x1 = "22" x2 = "22" ></ line > < line x1 = "6" y2 = "13" y1 = "11" x2 = "6" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;