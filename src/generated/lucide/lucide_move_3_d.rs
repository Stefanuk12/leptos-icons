use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 3v16h16" ></ path > < path d = "m5 19 6-6" ></ path > < path d = "m2 6 3-3 3 3" ></ path > < path d = "m18 16 3 3-3 3" ></ path > < / > } } pub const LUCIDE_MOVE_3_D : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;