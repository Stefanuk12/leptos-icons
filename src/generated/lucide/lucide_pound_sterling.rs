use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 7c0-5.333-8-5.333-8 0" ></ path > < path d = "M10 7v14" ></ path > < path d = "M6 21h12" ></ path > < path d = "M6 13h10" ></ path > < / > } } pub const LUCIDE_POUND_STERLING : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;