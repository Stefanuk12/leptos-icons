use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < rect width = "20" x = "2" rx = "2" height = "14" y = "3" ></ rect > < rect y = "7" x = "9" height = "6" rx = "1" width = "6" ></ rect > < / > } } pub const LUCIDE_MONITOR_STOP : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;