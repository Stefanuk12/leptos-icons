use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18.4 12c.8 3.8 2.6 5 2.6 5H3s3-2 3-9c0-3.3 2.7-6 6-6 1.8 0 3.4.8 4.5 2" ></ path > < path d = "M10.3 21a1.94 1.94 0 0 0 3.4 0" ></ path > < path d = "M15 8h6" ></ path > < / > } } pub const LUCIDE_BELL_MINUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;