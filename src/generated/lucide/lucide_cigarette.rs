use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 12H3a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h14" ></ path > < path d = "M18 8c0-2.5-2-2.5-2-5" ></ path > < path d = "M21 16a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1" ></ path > < path d = "M22 8c0-2.5-2-2.5-2-5" ></ path > < path d = "M7 12v4" ></ path > < / > } } pub const LUCIDE_CIGARETTE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;