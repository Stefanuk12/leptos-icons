use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" y = "7" height = "10" width = "16" x = "2" rx = "2" ></ rect > < line x2 = "22" x1 = "22" y1 = "11" y2 = "13" ></ line > < line y2 = "13" x1 = "6" x2 = "6" y1 = "11" ></ line > < line y2 = "13" y1 = "11" x2 = "10" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;