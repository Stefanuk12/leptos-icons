use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" y = "7" width = "16" height = "10" ry = "2" rx = "2" ></ rect > < line x1 = "22" x2 = "22" y1 = "11" y2 = "13" ></ line > < line x2 = "6" y1 = "11" y2 = "13" x1 = "6" ></ line > < line y2 = "13" x1 = "10" y1 = "11" x2 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;