use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8" ></ path > < path d = "M10 19v-3.96 3.15" ></ path > < path d = "M7 19h5" ></ path > < rect width = "6" y = "12" rx = "2" height = "10" x = "16" ></ rect > < / > } } pub const LUCIDE_MONITOR_SMARTPHONE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;