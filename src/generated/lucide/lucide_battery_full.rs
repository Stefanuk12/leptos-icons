use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" ry = "2" rx = "2" height = "10" x = "2" width = "16" ></ rect > < line y2 = "13" y1 = "11" x1 = "22" x2 = "22" ></ line > < line x2 = "6" y1 = "11" y2 = "13" x1 = "6" ></ line > < line x1 = "10" y1 = "11" y2 = "13" x2 = "10" ></ line > < line y1 = "11" x1 = "14" x2 = "14" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24")] } ;