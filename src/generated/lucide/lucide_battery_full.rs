use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "10" y = "7" rx = "2" x = "2" ry = "2" ></ rect > < line y1 = "11" x2 = "22" y2 = "13" x1 = "22" ></ line > < line y1 = "11" y2 = "13" x2 = "6" x1 = "6" ></ line > < line x1 = "10" y2 = "13" x2 = "10" y1 = "11" ></ line > < line x1 = "14" y2 = "13" y1 = "11" x2 = "14" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;