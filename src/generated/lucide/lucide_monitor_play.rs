use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m10 7 5 3-5 3Z" ></ path > < rect y = "3" height = "14" rx = "2" width = "20" x = "2" ></ rect > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < / > } } pub const LUCIDE_MONITOR_PLAY : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;