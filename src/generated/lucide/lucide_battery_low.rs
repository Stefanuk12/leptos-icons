use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" x = "2" width = "16" rx = "2" ry = "2" y = "7" ></ rect > < line y2 = "13" x1 = "22" y1 = "11" x2 = "22" ></ line > < line y1 = "11" y2 = "13" x2 = "6" x1 = "6" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;