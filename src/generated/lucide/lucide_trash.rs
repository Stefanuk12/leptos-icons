use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h18" ></ path > < path d = "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" ></ path > < path d = "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" ></ path > < / > } } pub const LUCIDE_TRASH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;