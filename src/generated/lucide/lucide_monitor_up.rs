use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 10 3-3 3 3" ></ path > < path d = "M12 13V7" ></ path > < rect x = "2" y = "3" rx = "2" width = "20" height = "14" ></ rect > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < / > } } pub const LUCIDE_MONITOR_UP : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;