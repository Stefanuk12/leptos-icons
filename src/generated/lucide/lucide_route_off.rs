use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 19h8.5c.4 0 .9-.1 1.3-.2" ></ path > < path d = "M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M21 15.3a3.5 3.5 0 0 0-3.3-3.3" ></ path > < path d = "M15 5h-4.3" ></ path > < circle r = "3" cx = "18" cy = "5" ></ circle > < / > } } pub const LucideRouteOff : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2")] } ;