use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "10" y = "7" ry = "2" x = "2" rx = "2" ></ rect > < line x1 = "22" y1 = "11" y2 = "13" x2 = "22" ></ line > < line y1 = "11" x1 = "6" x2 = "6" y2 = "13" ></ line > < line x1 = "10" y2 = "13" x2 = "10" y1 = "11" ></ line > < line y2 = "13" y1 = "11" x1 = "14" x2 = "14" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;