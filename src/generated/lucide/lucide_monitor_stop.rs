use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < rect y = "3" rx = "2" x = "2" width = "20" height = "14" ></ rect > < rect rx = "1" width = "6" x = "9" height = "6" y = "7" ></ rect > < / > } } pub const LUCIDE_MONITOR_STOP : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;