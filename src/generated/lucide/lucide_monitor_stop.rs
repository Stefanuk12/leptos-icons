use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < rect x = "2" rx = "2" width = "20" y = "3" height = "14" ></ rect > < rect y = "7" height = "6" rx = "1" width = "6" x = "9" ></ rect > < / > } } pub const LUCIDE_MONITOR_STOP : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;