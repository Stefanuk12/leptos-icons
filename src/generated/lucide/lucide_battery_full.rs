use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" rx = "2" height = "10" x = "2" width = "16" y = "7" ></ rect > < line y1 = "11" y2 = "13" x1 = "22" x2 = "22" ></ line > < line y2 = "13" y1 = "11" x1 = "6" x2 = "6" ></ line > < line y1 = "11" x2 = "10" x1 = "10" y2 = "13" ></ line > < line y2 = "13" x1 = "14" y1 = "11" x2 = "14" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;