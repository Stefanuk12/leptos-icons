use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" y = "7" rx = "2" ry = "2" x = "2" height = "10" ></ rect > < line y1 = "11" x2 = "22" x1 = "22" y2 = "13" ></ line > < line y2 = "13" x1 = "6" x2 = "6" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;