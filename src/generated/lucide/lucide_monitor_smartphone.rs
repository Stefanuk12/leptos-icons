use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8" ></ path > < path d = "M10 19v-3.96 3.15" ></ path > < path d = "M7 19h5" ></ path > < rect height = "10" y = "12" rx = "2" x = "16" width = "6" ></ rect > < / > } } pub const LUCIDE_MONITOR_SMARTPHONE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;