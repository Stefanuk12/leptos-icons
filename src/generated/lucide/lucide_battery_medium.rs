use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" x = "2" rx = "2" width = "16" ry = "2" height = "10" ></ rect > < line y2 = "13" y1 = "11" x1 = "22" x2 = "22" ></ line > < line x2 = "6" y2 = "13" x1 = "6" y1 = "11" ></ line > < line x2 = "10" y2 = "13" x1 = "10" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;