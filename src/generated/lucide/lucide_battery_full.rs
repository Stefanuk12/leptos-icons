use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" x = "2" height = "10" y = "7" width = "16" rx = "2" ></ rect > < line y1 = "11" x1 = "22" y2 = "13" x2 = "22" ></ line > < line y2 = "13" x2 = "6" x1 = "6" y1 = "11" ></ line > < line y1 = "11" x1 = "10" y2 = "13" x2 = "10" ></ line > < line x1 = "14" y2 = "13" x2 = "14" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;