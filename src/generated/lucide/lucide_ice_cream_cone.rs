use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 11 4.08 10.35a1 1 0 0 0 1.84 0L17 11" ></ path > < path d = "M17 7A5 5 0 0 0 7 7" ></ path > < path d = "M17 7a2 2 0 0 1 0 4H7a2 2 0 0 1 0-4" ></ path > < / > } } pub const LUCIDE_ICE_CREAM_CONE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24")] } ;