use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < rect height = "14" rx = "2" x = "2" y = "3" width = "20" ></ rect > < rect x = "9" width = "6" rx = "1" height = "6" y = "7" ></ rect > < / > } } pub const LUCIDE_MONITOR_STOP : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;