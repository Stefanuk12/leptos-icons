use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m5 9 7-7 7 7" ></ path > < path d = "M12 16V2" ></ path > < circle cy = "21" cx = "12" r = "1" ></ circle > < / > } } pub const LUCIDE_ARROW_UP_FROM_DOT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;