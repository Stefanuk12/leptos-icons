use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.5 20H8" ></ path > < path d = "M17 9h.01" ></ path > < rect width = "10" y = "4" height = "16" rx = "2" x = "12" ></ rect > < path d = "M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" ></ path > < circle cx = "17" cy = "15" r = "1" ></ circle > < / > } } pub const LUCIDE_MONITOR_SPEAKER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;