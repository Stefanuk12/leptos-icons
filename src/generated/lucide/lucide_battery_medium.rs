use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" y = "7" rx = "2" ry = "2" x = "2" width = "16" ></ rect > < line x2 = "22" x1 = "22" y1 = "11" y2 = "13" ></ line > < line x1 = "6" y1 = "11" y2 = "13" x2 = "6" ></ line > < line y1 = "11" y2 = "13" x1 = "10" x2 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;