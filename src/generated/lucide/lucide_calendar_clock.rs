use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.5" ></ path > < path d = "M16 2v4" ></ path > < path d = "M8 2v4" ></ path > < path d = "M3 10h5" ></ path > < path d = "M17.5 17.5 16 16.3V14" ></ path > < circle cy = "16" r = "6" cx = "16" ></ circle > < / > } } pub const LUCIDE_CALENDAR_CLOCK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;