use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" rx = "2" y = "7" height = "10" ry = "2" x = "2" ></ rect > < line y2 = "13" y1 = "11" x2 = "22" x1 = "22" ></ line > < line x2 = "6" y1 = "11" x1 = "6" y2 = "13" ></ line > < line x2 = "10" y2 = "13" y1 = "11" x1 = "10" ></ line > < line x1 = "14" x2 = "14" y2 = "13" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;