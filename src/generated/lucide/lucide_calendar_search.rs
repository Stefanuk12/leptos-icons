use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 12V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7.5" ></ path > < path d = "M16 2v4" ></ path > < path d = "M8 2v4" ></ path > < path d = "M3 10h18" ></ path > < circle cy = "18" r = "3" cx = "18" ></ circle > < path d = "m22 22-1.5-1.5" ></ path > < / > } } pub const LUCIDE_CALENDAR_SEARCH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;