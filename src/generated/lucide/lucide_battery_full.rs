use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "10" width = "16" y = "7" ry = "2" x = "2" ></ rect > < line x1 = "22" x2 = "22" y2 = "13" y1 = "11" ></ line > < line y1 = "11" y2 = "13" x2 = "6" x1 = "6" ></ line > < line x2 = "10" x1 = "10" y2 = "13" y1 = "11" ></ line > < line y1 = "11" x2 = "14" y2 = "13" x1 = "14" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;