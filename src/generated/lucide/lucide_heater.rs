use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 8c2-3-2-3 0-6" ></ path > < path d = "M15.5 8c2-3-2-3 0-6" ></ path > < path d = "M6 10h.01" ></ path > < path d = "M6 14h.01" ></ path > < path d = "M10 16v-4" ></ path > < path d = "M14 16v-4" ></ path > < path d = "M18 16v-4" ></ path > < path d = "M20 6a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3" ></ path > < path d = "M5 20v2" ></ path > < path d = "M19 20v2" ></ path > < / > } } pub const LUCIDE_HEATER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round")] } ;