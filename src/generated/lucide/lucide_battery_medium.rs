use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "10" x = "2" y = "7" rx = "2" ry = "2" ></ rect > < line x2 = "22" y1 = "11" x1 = "22" y2 = "13" ></ line > < line y1 = "11" x1 = "6" y2 = "13" x2 = "6" ></ line > < line y2 = "13" y1 = "11" x2 = "10" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;