use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" ry = "2" width = "16" y = "7" height = "10" rx = "2" ></ rect > < line x1 = "22" y1 = "11" y2 = "13" x2 = "22" ></ line > < line x1 = "6" y1 = "11" y2 = "13" x2 = "6" ></ line > < line x1 = "10" x2 = "10" y1 = "11" y2 = "13" ></ line > < line y2 = "13" x1 = "14" x2 = "14" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;