use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "16" x = "2" height = "10" y = "7" rx = "2" ></ rect > < line x1 = "22" y1 = "11" x2 = "22" y2 = "13" ></ line > < line y2 = "13" y1 = "11" x2 = "6" x1 = "6" ></ line > < line x2 = "10" x1 = "10" y1 = "11" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;