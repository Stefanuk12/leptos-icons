use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.1 2.182a10 10 0 0 1 3.8 0" ></ path > < path d = "M13.9 21.818a10 10 0 0 1-3.8 0" ></ path > < path d = "M17.609 3.721a10 10 0 0 1 2.69 2.7" ></ path > < path d = "M2.182 13.9a10 10 0 0 1 0-3.8" ></ path > < path d = "M20.279 17.609a10 10 0 0 1-2.7 2.69" ></ path > < path d = "M21.818 10.1a10 10 0 0 1 0 3.8" ></ path > < path d = "M3.721 6.391a10 10 0 0 1 2.7-2.69" ></ path > < path d = "M6.391 20.279a10 10 0 0 1-2.69-2.7" ></ path > < / > } } pub const LUCIDE_CIRCLE_DASHED : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2")] } ;