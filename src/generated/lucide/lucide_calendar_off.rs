use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4.2 4.2A2 2 0 0 0 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 1.82-1.18" ></ path > < path d = "M21 15.5V6a2 2 0 0 0-2-2H9.5" ></ path > < path d = "M16 2v4" ></ path > < path d = "M3 10h7" ></ path > < path d = "M21 10h-5.5" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_CALENDAR_OFF : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;