use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72" ></ path > < path d = "m14 7 3 3" ></ path > < path d = "M5 6v4" ></ path > < path d = "M19 14v4" ></ path > < path d = "M10 2v2" ></ path > < path d = "M7 8H3" ></ path > < path d = "M21 16h-4" ></ path > < path d = "M11 3H9" ></ path > < / > } } pub const LUCIDE_WAND_SPARKLES : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;