use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v14" ></ path > < path d = "m19 9-7 7-7-7" ></ path > < circle cy = "21" r = "1" cx = "12" ></ circle > < / > } } pub const LUCIDE_ARROW_DOWN_TO_DOT : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;