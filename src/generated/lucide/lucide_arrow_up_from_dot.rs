use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m5 9 7-7 7 7" ></ path > < path d = "M12 16V2" ></ path > < circle cy = "21" r = "1" cx = "12" ></ circle > < / > } } pub const LUCIDE_ARROW_UP_FROM_DOT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24")] } ;