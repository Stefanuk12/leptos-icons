use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 10.821-7.487" ></ path > < path d = "M21.378 16.626a1 1 0 0 0-3.004-3.004l-4.01 4.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z" ></ path > < circle cy = "8" cx = "10" r = "5" ></ circle > < / > } } pub const LUCIDE_USER_ROUND_PEN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;