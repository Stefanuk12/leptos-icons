use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" x = "2" ry = "2" height = "10" rx = "2" width = "16" ></ rect > < line y1 = "11" y2 = "13" x1 = "22" x2 = "22" ></ line > < line y1 = "11" x1 = "6" x2 = "6" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;