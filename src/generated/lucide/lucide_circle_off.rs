use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 2 20 20" ></ path > < path d = "M8.35 2.69A10 10 0 0 1 21.3 15.65" ></ path > < path d = "M19.08 19.08A10 10 0 1 1 4.92 4.92" ></ path > < / > } } pub const LUCIDE_CIRCLE_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;