use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" y = "7" rx = "2" height = "10" ry = "2" x = "2" ></ rect > < line x1 = "22" y1 = "11" y2 = "13" x2 = "22" ></ line > < line y1 = "11" y2 = "13" x1 = "6" x2 = "6" ></ line > < line y1 = "11" y2 = "13" x2 = "10" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;