use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" rx = "2" height = "10" width = "16" x = "2" ry = "2" ></ rect > < line x1 = "22" y2 = "13" y1 = "11" x2 = "22" ></ line > < line y2 = "13" x2 = "6" x1 = "6" y1 = "11" ></ line > < line y2 = "13" x1 = "10" y1 = "11" x2 = "10" ></ line > < line x1 = "14" x2 = "14" y2 = "13" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;