use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 3H5a2 2 0 0 0-2 2v3" ></ path > < path d = "M21 8V5a2 2 0 0 0-2-2h-3" ></ path > < path d = "M3 16v3a2 2 0 0 0 2 2h3" ></ path > < path d = "M16 21h3a2 2 0 0 0 2-2v-3" ></ path > < / > } } pub const LUCIDE_MAXIMIZE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none")] } ;