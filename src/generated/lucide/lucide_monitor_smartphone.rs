use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8" ></ path > < path d = "M10 19v-3.96 3.15" ></ path > < path d = "M7 19h5" ></ path > < rect x = "16" rx = "2" y = "12" height = "10" width = "6" ></ rect > < / > } } pub const LUCIDE_MONITOR_SMARTPHONE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;