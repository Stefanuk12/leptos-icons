use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" y = "7" height = "10" rx = "2" x = "2" ry = "2" ></ rect > < line x1 = "22" y1 = "11" x2 = "22" y2 = "13" ></ line > < line x2 = "6" x1 = "6" y2 = "13" y1 = "11" ></ line > < line y1 = "11" x1 = "10" y2 = "13" x2 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;