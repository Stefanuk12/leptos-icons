use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" y = "7" rx = "2" x = "2" height = "10" ry = "2" ></ rect > < line y2 = "13" x1 = "22" y1 = "11" x2 = "22" ></ line > < line y1 = "11" x1 = "6" x2 = "6" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;