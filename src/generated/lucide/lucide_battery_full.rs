use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "2" width = "16" ry = "2" y = "7" height = "10" ></ rect > < line y2 = "13" x2 = "22" y1 = "11" x1 = "22" ></ line > < line y1 = "11" x2 = "6" x1 = "6" y2 = "13" ></ line > < line y1 = "11" x1 = "10" x2 = "10" y2 = "13" ></ line > < line y1 = "11" x1 = "14" y2 = "13" x2 = "14" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;