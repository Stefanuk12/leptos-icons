use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "16" x = "2" y = "7" height = "10" rx = "2" ></ rect > < line x1 = "22" y1 = "11" y2 = "13" x2 = "22" ></ line > < line x2 = "6" y1 = "11" y2 = "13" x1 = "6" ></ line > < line y1 = "11" x1 = "10" y2 = "13" x2 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24")] } ;