use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v14" ></ path > < path d = "m19 9-7 7-7-7" ></ path > < circle cx = "12" cy = "21" r = "1" ></ circle > < / > } } pub const LUCIDE_ARROW_DOWN_TO_DOT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;