use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "6" cy = "19" ></ circle > < path d = "M9 19h8.5c.4 0 .9-.1 1.3-.2" ></ path > < path d = "M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M21 15.3a3.5 3.5 0 0 0-3.3-3.3" ></ path > < path d = "M15 5h-4.3" ></ path > < circle cy = "5" r = "3" cx = "18" ></ circle > < / > } } pub const LUCIDE_ROUTE_OFF : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2")] } ;