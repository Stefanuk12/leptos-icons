use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.5 20H8" ></ path > < path d = "M17 9h.01" ></ path > < rect width = "10" rx = "2" x = "12" height = "16" y = "4" ></ rect > < path d = "M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" ></ path > < circle r = "1" cx = "17" cy = "15" ></ circle > < / > } } pub const LUCIDE_MONITOR_SPEAKER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;