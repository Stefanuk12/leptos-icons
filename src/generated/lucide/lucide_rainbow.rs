use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17a10 10 0 0 0-20 0" ></ path > < path d = "M6 17a6 6 0 0 1 12 0" ></ path > < path d = "M10 17a2 2 0 0 1 4 0" ></ path > < / > } } pub const LUCIDE_RAINBOW : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;