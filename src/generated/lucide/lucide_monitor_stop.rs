use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < rect width = "20" height = "14" rx = "2" y = "3" x = "2" ></ rect > < rect height = "6" x = "9" rx = "1" width = "6" y = "7" ></ rect > < / > } } pub const LUCIDE_MONITOR_STOP : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;