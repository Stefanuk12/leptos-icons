use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 17H4a2 2 0 0 1-2-2V5c0-1.5 1-2 1-2" ></ path > < path d = "M22 15V5a2 2 0 0 0-2-2H9" ></ path > < path d = "M8 21h8" ></ path > < path d = "M12 17v4" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_MONITOR_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;