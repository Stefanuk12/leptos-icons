use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "10" width = "16" y = "7" x = "2" ry = "2" ></ rect > < line x2 = "22" x1 = "22" y1 = "11" y2 = "13" ></ line > < line y1 = "11" x1 = "6" x2 = "6" y2 = "13" ></ line > < line x1 = "10" y1 = "11" x2 = "10" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;