use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z" ></ path > < path d = "m2 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z" ></ path > < path d = "M7 21h10" ></ path > < path d = "M12 3v18" ></ path > < path d = "M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2" ></ path > < / > } } pub const LUCIDE_SCALE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;