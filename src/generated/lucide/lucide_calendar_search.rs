use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 2v4" ></ path > < path d = "M21 11.75V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7.25" ></ path > < path d = "m22 22-1.875-1.875" ></ path > < path d = "M3 10h18" ></ path > < path d = "M8 2v4" ></ path > < circle cy = "18" cx = "18" r = "3" ></ circle > < / > } } pub const LUCIDE_CALENDAR_SEARCH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24")] } ;