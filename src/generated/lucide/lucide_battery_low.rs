use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "10" x = "2" ry = "2" width = "16" y = "7" ></ rect > < line y1 = "11" x2 = "22" x1 = "22" y2 = "13" ></ line > < line x1 = "6" x2 = "6" y1 = "11" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2")] } ;