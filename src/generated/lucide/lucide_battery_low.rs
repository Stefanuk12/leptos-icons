use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "2" ry = "2" width = "16" height = "10" y = "7" ></ rect > < line x2 = "22" y1 = "11" y2 = "13" x1 = "22" ></ line > < line x1 = "6" y1 = "11" x2 = "6" y2 = "13" ></ line > < / > } } pub const LUCIDE_BATTERY_LOW : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;