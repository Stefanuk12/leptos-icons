use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 8a2 2 0 0 1-2-2V3h20v3a2 2 0 0 1-2 2Z" ></ path > < path d = "m19 8-.8 3c-.1.6-.6 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L5 8" ></ path > < path d = "M16 21c0-2.5 2-2.5 2-5" ></ path > < path d = "M11 21c0-2.5 2-2.5 2-5" ></ path > < path d = "M6 21c0-2.5 2-2.5 2-5" ></ path > < / > } } pub const LUCIDE_ALARM_SMOKE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;