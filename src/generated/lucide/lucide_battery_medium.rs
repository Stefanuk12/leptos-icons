use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" ry = "2" height = "10" x = "2" rx = "2" y = "7" ></ rect > < line y1 = "11" x1 = "22" y2 = "13" x2 = "22" ></ line > < line y2 = "13" y1 = "11" x1 = "6" x2 = "6" ></ line > < line y2 = "13" x1 = "10" y1 = "11" x2 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24")] } ;