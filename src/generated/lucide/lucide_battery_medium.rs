use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" width = "16" x = "2" ry = "2" y = "7" rx = "2" ></ rect > < line x1 = "22" y1 = "11" x2 = "22" y2 = "13" ></ line > < line x2 = "6" y1 = "11" y2 = "13" x1 = "6" ></ line > < line x1 = "10" x2 = "10" y1 = "11" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;