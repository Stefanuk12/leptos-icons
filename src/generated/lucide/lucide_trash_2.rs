use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h18" ></ path > < path d = "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" ></ path > < path d = "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" ></ path > < line x1 = "10" x2 = "10" y2 = "17" y1 = "11" ></ line > < line y2 = "17" y1 = "11" x1 = "14" x2 = "14" ></ line > < / > } } pub const LUCIDE_TRASH_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;