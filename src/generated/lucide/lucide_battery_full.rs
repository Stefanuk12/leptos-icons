use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" rx = "2" x = "2" y = "7" height = "10" ry = "2" ></ rect > < line x2 = "22" y1 = "11" y2 = "13" x1 = "22" ></ line > < line y1 = "11" x1 = "6" x2 = "6" y2 = "13" ></ line > < line y1 = "11" x2 = "10" y2 = "13" x1 = "10" ></ line > < line y2 = "13" x1 = "14" y1 = "11" x2 = "14" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24")] } ;