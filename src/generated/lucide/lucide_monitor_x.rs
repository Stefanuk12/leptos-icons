use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m14.5 12.5-5-5" ></ path > < path d = "m9.5 12.5 5-5" ></ path > < rect x = "2" height = "14" rx = "2" width = "20" y = "3" ></ rect > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < / > } } pub const LUCIDE_MONITOR_X : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;