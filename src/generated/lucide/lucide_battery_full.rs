use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" x = "2" rx = "2" ry = "2" width = "16" y = "7" ></ rect > < line x2 = "22" y1 = "11" y2 = "13" x1 = "22" ></ line > < line x1 = "6" x2 = "6" y1 = "11" y2 = "13" ></ line > < line x2 = "10" x1 = "10" y2 = "13" y1 = "11" ></ line > < line y2 = "13" y1 = "11" x2 = "14" x1 = "14" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2")] } ;