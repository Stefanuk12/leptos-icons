use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12h20" ></ path > < path d = "M10 16v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4" ></ path > < path d = "M10 8V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v4" ></ path > < path d = "M20 16v1a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-1" ></ path > < path d = "M14 8V7c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v1" ></ path > < / > } } pub const LUCIDE_ALIGN_CENTER_HORIZONTAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;