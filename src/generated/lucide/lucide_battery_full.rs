use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" height = "10" rx = "2" width = "16" x = "2" y = "7" ></ rect > < line x2 = "22" x1 = "22" y2 = "13" y1 = "11" ></ line > < line x1 = "6" x2 = "6" y2 = "13" y1 = "11" ></ line > < line y1 = "11" x1 = "10" x2 = "10" y2 = "13" ></ line > < line x1 = "14" x2 = "14" y1 = "11" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none")] } ;