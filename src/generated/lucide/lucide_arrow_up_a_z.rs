use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < path d = "M20 8h-5" ></ path > < path d = "M15 10V6.5a2.5 2.5 0 0 1 5 0V10" ></ path > < path d = "M15 14h5l-5 6h5" ></ path > < / > } } pub const LUCIDE_ARROW_UP_A_Z : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;