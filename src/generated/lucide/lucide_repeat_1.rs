use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m17 2 4 4-4 4" ></ path > < path d = "M3 11v-1a4 4 0 0 1 4-4h14" ></ path > < path d = "m7 22-4-4 4-4" ></ path > < path d = "M21 13v1a4 4 0 0 1-4 4H3" ></ path > < path d = "M11 10h1v4" ></ path > < / > } } pub const LUCIDE_REPEAT_1 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;