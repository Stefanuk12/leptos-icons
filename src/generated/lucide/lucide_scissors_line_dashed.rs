use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.42 9.42 8 12" ></ path > < circle cy = "8" r = "2" cx = "4" ></ circle > < path d = "m14 6-8.58 8.58" ></ path > < circle cx = "4" cy = "16" r = "2" ></ circle > < path d = "M10.8 14.8 14 18" ></ path > < path d = "M16 12h-2" ></ path > < path d = "M22 12h-2" ></ path > < / > } } pub const LUCIDE_SCISSORS_LINE_DASHED : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;