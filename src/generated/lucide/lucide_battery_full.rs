use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" width = "16" ry = "2" y = "7" rx = "2" x = "2" ></ rect > < line y1 = "11" x1 = "22" y2 = "13" x2 = "22" ></ line > < line y1 = "11" y2 = "13" x2 = "6" x1 = "6" ></ line > < line x1 = "10" y1 = "11" x2 = "10" y2 = "13" ></ line > < line x2 = "14" x1 = "14" y2 = "13" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;