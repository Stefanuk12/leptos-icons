use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 4v16" ></ path > < path d = "M15 4h5l-5 6h5" ></ path > < path d = "M15 20v-3.5a2.5 2.5 0 0 1 5 0V20" ></ path > < path d = "M20 18h-5" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_Z_A : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;