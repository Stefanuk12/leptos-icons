use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 12H3a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h14" ></ path > < path d = "M18 8c0-2.5-2-2.5-2-5" ></ path > < path d = "M21 16a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1" ></ path > < path d = "M22 8c0-2.5-2-2.5-2-5" ></ path > < path d = "M7 12v4" ></ path > < / > } } pub const LUCIDE_CIGARETTE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;