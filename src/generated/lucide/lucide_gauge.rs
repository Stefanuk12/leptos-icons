use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 14 4-4" ></ path > < path d = "M3.34 19a10 10 0 1 1 17.32 0" ></ path > < / > } } pub const LUCIDE_GAUGE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;