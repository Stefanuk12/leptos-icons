use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v14" ></ path > < path d = "m19 9-7 7-7-7" ></ path > < circle cx = "12" r = "1" cy = "21" ></ circle > < / > } } pub const LUCIDE_ARROW_DOWN_TO_DOT : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;