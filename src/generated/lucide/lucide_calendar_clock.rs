use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.5" ></ path > < path d = "M16 2v4" ></ path > < path d = "M8 2v4" ></ path > < path d = "M3 10h5" ></ path > < path d = "M17.5 17.5 16 16.3V14" ></ path > < circle r = "6" cx = "16" cy = "16" ></ circle > < / > } } pub const LucideCalendarClock : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round")] } ;