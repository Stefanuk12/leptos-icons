use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 19h6" ></ path > < path d = "M16 2v4" ></ path > < path d = "M21 15V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8.5" ></ path > < path d = "M3 10h18" ></ path > < path d = "M8 2v4" ></ path > < / > } } pub const LUCIDE_CALENDAR_MINUS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;