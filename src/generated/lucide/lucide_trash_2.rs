use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h18" ></ path > < path d = "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" ></ path > < path d = "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" ></ path > < line x1 = "10" y1 = "11" x2 = "10" y2 = "17" ></ line > < line y1 = "11" x2 = "14" y2 = "17" x1 = "14" ></ line > < / > } } pub const LUCIDE_TRASH_2 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;