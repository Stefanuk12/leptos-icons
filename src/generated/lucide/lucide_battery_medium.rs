use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "10" y = "7" rx = "2" x = "2" ry = "2" ></ rect > < line y1 = "11" x2 = "22" y2 = "13" x1 = "22" ></ line > < line y2 = "13" x2 = "6" y1 = "11" x1 = "6" ></ line > < line x2 = "10" y1 = "11" y2 = "13" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;