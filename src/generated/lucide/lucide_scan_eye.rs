use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 7V5a2 2 0 0 1 2-2h2" ></ path > < path d = "M17 3h2a2 2 0 0 1 2 2v2" ></ path > < path d = "M21 17v2a2 2 0 0 1-2 2h-2" ></ path > < path d = "M7 21H5a2 2 0 0 1-2-2v-2" ></ path > < circle r = "1" cx = "12" cy = "12" ></ circle > < path d = "M5 12s2.5-5 7-5 7 5 7 5-2.5 5-7 5-7-5-7-5" ></ path > < / > } } pub const LUCIDE_SCAN_EYE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;