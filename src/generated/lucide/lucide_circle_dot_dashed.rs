use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.1 2.18a9.93 9.93 0 0 1 3.8 0" ></ path > < path d = "M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7" ></ path > < path d = "M21.82 10.1a9.93 9.93 0 0 1 0 3.8" ></ path > < path d = "M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69" ></ path > < path d = "M13.9 21.82a9.94 9.94 0 0 1-3.8 0" ></ path > < path d = "M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7" ></ path > < path d = "M2.18 13.9a9.93 9.93 0 0 1 0-3.8" ></ path > < path d = "M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69" ></ path > < circle cx = "12" cy = "12" r = "1" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DOT_DASHED : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;