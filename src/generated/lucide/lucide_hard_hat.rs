use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 18a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v2z" ></ path > < path d = "M10 10V5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5" ></ path > < path d = "M4 15v-3a6 6 0 0 1 6-6h0" ></ path > < path d = "M14 6h0a6 6 0 0 1 6 6v3" ></ path > < / > } } pub const LUCIDE_HARD_HAT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;