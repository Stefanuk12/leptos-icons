use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.34 17.52a10 10 0 1 0-2.82 2.82" ></ path > < circle cy = "19" r = "2" cx = "19" ></ circle > < path d = "m13.41 13.41 4.18 4.18" ></ path > < circle cy = "12" cx = "12" r = "2" ></ circle > < / > } } pub const LUCIDE_RADIUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;