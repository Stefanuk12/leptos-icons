use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 7V5a2 2 0 0 1 2-2h2" ></ path > < path d = "M17 3h2a2 2 0 0 1 2 2v2" ></ path > < path d = "M21 17v2a2 2 0 0 1-2 2h-2" ></ path > < path d = "M7 21H5a2 2 0 0 1-2-2v-2" ></ path > < path d = "M7 8h8" ></ path > < path d = "M7 12h10" ></ path > < path d = "M7 16h6" ></ path > < / > } } pub const LUCIDE_SCAN_TEXT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;