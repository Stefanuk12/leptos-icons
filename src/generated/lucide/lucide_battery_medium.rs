use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" rx = "2" height = "10" y = "7" width = "16" ry = "2" ></ rect > < line y1 = "11" x2 = "22" y2 = "13" x1 = "22" ></ line > < line y1 = "11" x1 = "6" y2 = "13" x2 = "6" ></ line > < line x1 = "10" y1 = "11" y2 = "13" x2 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;