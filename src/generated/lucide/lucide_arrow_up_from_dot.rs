use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m5 9 7-7 7 7" ></ path > < path d = "M12 16V2" ></ path > < circle r = "1" cx = "12" cy = "21" ></ circle > < / > } } pub const LUCIDE_ARROW_UP_FROM_DOT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24")] } ;