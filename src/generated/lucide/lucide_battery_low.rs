use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" rx = "2" height = "10" width = "16" x = "2" y = "7" ></ rect > < line y2 = "13" x2 = "22" x1 = "22" y1 = "11" ></ line > < line x1 = "6" x2 = "6" y1 = "11" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;