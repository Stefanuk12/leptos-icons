use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 12V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7.5" ></ path > < path d = "M16 2v4" ></ path > < path d = "M8 2v4" ></ path > < path d = "M3 10h18" ></ path > < circle cx = "18" cy = "18" r = "3" ></ circle > < path d = "m22 22-1.5-1.5" ></ path > < / > } } pub const LUCIDE_CALENDAR_SEARCH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;