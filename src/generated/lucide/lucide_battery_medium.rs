use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" y = "7" height = "10" x = "2" rx = "2" ry = "2" ></ rect > < line x2 = "22" y2 = "13" x1 = "22" y1 = "11" ></ line > < line x2 = "6" y2 = "13" x1 = "6" y1 = "11" ></ line > < line x1 = "10" x2 = "10" y2 = "13" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_MEDIUM : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;