use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" ry = "2" height = "10" width = "16" y = "7" rx = "2" ></ rect > < line x1 = "22" x2 = "22" y1 = "11" y2 = "13" ></ line > < line x2 = "6" y1 = "11" y2 = "13" x1 = "6" ></ line > < line y1 = "11" x1 = "10" x2 = "10" y2 = "13" ></ line > < line x2 = "14" y1 = "11" y2 = "13" x1 = "14" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;