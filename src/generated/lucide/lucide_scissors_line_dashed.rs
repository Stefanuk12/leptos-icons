use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.42 9.42 8 12" ></ path > < circle cx = "4" r = "2" cy = "8" ></ circle > < path d = "m14 6-8.58 8.58" ></ path > < circle cy = "16" r = "2" cx = "4" ></ circle > < path d = "M10.8 14.8 14 18" ></ path > < path d = "M16 12h-2" ></ path > < path d = "M22 12h-2" ></ path > < / > } } pub const LUCIDE_SCISSORS_LINE_DASHED : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;