use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < path d = "M11 12h4" ></ path > < path d = "M11 16h7" ></ path > < path d = "M11 20h10" ></ path > < / > } } pub const LUCIDE_ARROW_UP_NARROW_WIDE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;