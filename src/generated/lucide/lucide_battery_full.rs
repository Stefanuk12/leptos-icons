use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" x = "2" ry = "2" rx = "2" height = "10" y = "7" ></ rect > < line x2 = "22" x1 = "22" y1 = "11" y2 = "13" ></ line > < line y2 = "13" x1 = "6" x2 = "6" y1 = "11" ></ line > < line x1 = "10" x2 = "10" y1 = "11" y2 = "13" ></ line > < line x2 = "14" x1 = "14" y2 = "13" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24")] } ;