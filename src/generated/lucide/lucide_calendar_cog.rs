use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15.2 16.9-.9-.4" ></ path > < path d = "m15.2 19.1-.9.4" ></ path > < path d = "M16 2v4" ></ path > < path d = "m16.9 15.2-.4-.9" ></ path > < path d = "m16.9 20.8-.4.9" ></ path > < path d = "m19.5 14.3-.4.9" ></ path > < path d = "m19.5 21.7-.4-.9" ></ path > < path d = "M21 10.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6" ></ path > < path d = "m21.7 16.5-.9.4" ></ path > < path d = "m21.7 19.5-.9-.4" ></ path > < path d = "M3 10h18" ></ path > < path d = "M8 2v4" ></ path > < circle cx = "18" cy = "18" r = "3" ></ circle > < / > } } pub const LUCIDE_CALENDAR_COG : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24")] } ;