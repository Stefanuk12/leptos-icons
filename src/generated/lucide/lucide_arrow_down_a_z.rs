use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 20V4" ></ path > < path d = "M20 8h-5" ></ path > < path d = "M15 10V6.5a2.5 2.5 0 0 1 5 0V10" ></ path > < path d = "M15 14h5l-5 6h5" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_A_Z : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;