use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17a10 10 0 0 0-20 0" ></ path > < path d = "M6 17a6 6 0 0 1 12 0" ></ path > < path d = "M10 17a2 2 0 0 1 4 0" ></ path > < / > } } pub const LUCIDE_RAINBOW : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;