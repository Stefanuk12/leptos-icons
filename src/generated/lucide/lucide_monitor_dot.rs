use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "19" cy = "6" ></ circle > < path d = "M22 12v3a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h9" ></ path > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < / > } } pub const LUCIDE_MONITOR_DOT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;