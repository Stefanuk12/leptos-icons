use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.5 20H8" ></ path > < path d = "M17 9h.01" ></ path > < rect x = "12" width = "10" height = "16" y = "4" rx = "2" ></ rect > < path d = "M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" ></ path > < circle cy = "15" cx = "17" r = "1" ></ circle > < / > } } pub const LUCIDE_MONITOR_SPEAKER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;