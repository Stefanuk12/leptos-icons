use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < path d = "M11 12h10" ></ path > < path d = "M11 16h7" ></ path > < path d = "M11 20h4" ></ path > < / > } } pub const LUCIDE_ARROW_UP_WIDE_NARROW : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24")] } ;