use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v3" ></ path > < path d = "M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3" ></ path > < path d = "M4 12H2" ></ path > < path d = "M10 12H8" ></ path > < path d = "M16 12h-2" ></ path > < path d = "M22 12h-2" ></ path > < / > } } pub const LUCIDE_FLIP_VERTICAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;