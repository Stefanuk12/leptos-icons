use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17v4" ></ path > < path d = "m15.2 4.9-.9-.4" ></ path > < path d = "m15.2 7.1-.9.4" ></ path > < path d = "m16.9 3.2-.4-.9" ></ path > < path d = "m16.9 8.8-.4.9" ></ path > < path d = "m19.5 2.3-.4.9" ></ path > < path d = "m19.5 9.7-.4-.9" ></ path > < path d = "m21.7 4.5-.9.4" ></ path > < path d = "m21.7 7.5-.9-.4" ></ path > < path d = "M22 13v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7" ></ path > < path d = "M8 21h8" ></ path > < circle cy = "6" r = "3" cx = "18" ></ circle > < / > } } pub const LUCIDE_MONITOR_COG : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;