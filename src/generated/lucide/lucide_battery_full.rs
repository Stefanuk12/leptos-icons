use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" width = "16" height = "10" x = "2" y = "7" ></ rect > < line x2 = "22" y2 = "13" x1 = "22" y1 = "11" ></ line > < line x2 = "6" y1 = "11" x1 = "6" y2 = "13" ></ line > < line x2 = "10" x1 = "10" y1 = "11" y2 = "13" ></ line > < line x1 = "14" x2 = "14" y1 = "11" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;