use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" x = "2" height = "10" width = "16" y = "7" rx = "2" ></ rect > < line y2 = "13" x1 = "22" x2 = "22" y1 = "11" ></ line > < line y2 = "13" y1 = "11" x1 = "6" x2 = "6" ></ line > < line x1 = "10" y2 = "13" y1 = "11" x2 = "10" ></ line > < line x1 = "14" y1 = "11" x2 = "14" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;