use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M12 2v4" ></ path > < path d = "M16 2v4" ></ path > < path d = "M16 4h2a2 2 0 0 1 2 2v2" ></ path > < path d = "M20 12v2" ></ path > < path d = "M20 18v2a2 2 0 0 1-2 2h-1" ></ path > < path d = "M13 22h-2" ></ path > < path d = "M7 22H6a2 2 0 0 1-2-2v-2" ></ path > < path d = "M4 14v-2" ></ path > < path d = "M4 8V6a2 2 0 0 1 2-2h2" ></ path > < path d = "M8 10h6" ></ path > < path d = "M8 14h8" ></ path > < path d = "M8 18h5" ></ path > < / > } } pub const LUCIDE_NOTEPAD_TEXT_DASHED : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;