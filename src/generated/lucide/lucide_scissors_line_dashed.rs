use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.42 9.42 8 12" ></ path > < circle cx = "4" r = "2" cy = "8" ></ circle > < path d = "m14 6-8.58 8.58" ></ path > < circle cy = "16" cx = "4" r = "2" ></ circle > < path d = "M10.8 14.8 14 18" ></ path > < path d = "M16 12h-2" ></ path > < path d = "M22 12h-2" ></ path > < / > } } pub const LUCIDE_SCISSORS_LINE_DASHED : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;