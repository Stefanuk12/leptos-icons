use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "2" ry = "2" width = "16" height = "10" y = "7" ></ rect > < line x2 = "22" y2 = "13" y1 = "11" x1 = "22" ></ line > < line y2 = "13" x2 = "6" x1 = "6" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;