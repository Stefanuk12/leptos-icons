use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 3v3a2 2 0 0 1-2 2H3" ></ path > < path d = "M21 8h-3a2 2 0 0 1-2-2V3" ></ path > < path d = "M3 16h3a2 2 0 0 1 2 2v3" ></ path > < path d = "M16 21v-3a2 2 0 0 1 2-2h3" ></ path > < / > } } pub const LUCIDE_MINIMIZE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;