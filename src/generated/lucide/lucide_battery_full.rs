use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" y = "7" ry = "2" x = "2" rx = "2" width = "16" ></ rect > < line y2 = "13" y1 = "11" x1 = "22" x2 = "22" ></ line > < line y2 = "13" y1 = "11" x2 = "6" x1 = "6" ></ line > < line y2 = "13" x2 = "10" y1 = "11" x1 = "10" ></ line > < line x2 = "14" y2 = "13" x1 = "14" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;