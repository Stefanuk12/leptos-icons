use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.5 20H8" ></ path > < path d = "M17 9h.01" ></ path > < rect rx = "2" y = "4" width = "10" height = "16" x = "12" ></ rect > < path d = "M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" ></ path > < circle cx = "17" r = "1" cy = "15" ></ circle > < / > } } pub const LUCIDE_MONITOR_SPEAKER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;