use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.34 17.52a10 10 0 1 0-2.82 2.82" ></ path > < circle cx = "19" cy = "19" r = "2" ></ circle > < path d = "m13.41 13.41 4.18 4.18" ></ path > < circle cx = "12" r = "2" cy = "12" ></ circle > < / > } } pub const LUCIDE_RADIUS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;