use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h18" ></ path > < path d = "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" ></ path > < path d = "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" ></ path > < line x2 = "10" y1 = "11" y2 = "17" x1 = "10" ></ line > < line y1 = "11" x2 = "14" x1 = "14" y2 = "17" ></ line > < / > } } pub const LUCIDE_TRASH_2 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;