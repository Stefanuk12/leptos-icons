use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m14 12-8.5 8.5a2.12 2.12 0 1 1-3-3L11 9" ></ path > < path d = "M15 13 9 7l4-4 6 6h3a8 8 0 0 1-7 7z" ></ path > < / > } } pub const LUCIDE_AXE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;